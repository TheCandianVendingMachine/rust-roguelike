/*
    a roguelike game created for a fun exercise
    copyright (c) 2023  bailey danyluk

    this program is free software: you can redistribute it and/or modify
    it under the terms of the gnu general public license as published by
    the free software foundation, either version 3 of the license, or
    (at your option) any later version.

    this program is distributed in the hope that it will be useful,
    but without any warranty; without even the implied warranty of
    merchantability or fitness for a particular purpose.  see the
    gnu general public license for more details.

    you should have received a copy of the gnu general public license
    along with this program.  if not, see <https://www.gnu.org/licenses/>.
*/

pub mod benchmark {
    use std::io::{ Write, BufWriter };
    use std::os::unix::io::{ FromRawFd, IntoRawFd };
    use std::fs::File;
    use std::thread;
    use std::sync::{ Arc, RwLock, atomic::AtomicU8 };
    use log::debug;
    use std::time::{Instant, Duration};
    use crate::renderer::colours::Colour;
    use crate::renderer::crossterm::{ 
        colour,
        device_settings::DeviceSettings,
        render_engine::RenderEngine,
        command_buffer::CommandBuffer, command::{ Command, ClearInfo, DrawInfo },
        swapchain::Swapchain
    };
    use std::sync::atomic::Ordering;

    /// Test rendering terminal in multiple colours. Colours will be rendered in different
    /// moments to minimize characters drawn, characters of same colour drawn, etc
    fn command_buffer_usage() -> Duration {
        let mut accumulator = Duration::ZERO;

        let pos = (40, 5);
        let mut writer = BufWriter::new(unsafe { File::from_raw_fd(1) });

        let mut command_buffer = CommandBuffer::new();
        let mut command_creator = command_buffer.add_commands().execute(Command::Clear(ClearInfo::All));
        accumulator += {
            let colours = [Colour::red(), Colour::green(), Colour::dark_purple(), Colour::red(), Colour::blue(), Colour::white(), Colour::rgb(40, 67, 128)];
            let t0 = Instant::now();
            for y in 0..13 {
                for x in 0..10 {
                    command_creator = command_creator.execute(Command::Draw(DrawInfo {
                        colour: colour::map_to_crossterm_rgb(&colours[(y + x * x) as usize % colours.len()]),
                        draw_pos_x: pos.0 + x,
                        draw_pos_y: pos.1 + y,
                        character: 'X'
                    }));
                }
            }
            // command_creator = command_creator.execute(Command::Flush);
            let elapsed = t0.elapsed();
            debug!("Draw Command Created: {} micro-seconds", elapsed.as_micros());
            elapsed
        };
        accumulator += {
            let t0 = Instant::now();
            command_creator.compile();
            let elapsed = t0.elapsed();
            debug!("Command Buffer Compilation: {} micro-seconds", elapsed.as_micros());
            elapsed
        };

        accumulator += {
            let t0 = Instant::now();
            command_buffer.execute(&mut writer);
            let elapsed = t0.elapsed();
            debug!("Command Buffer Execution: {} micro-seconds", elapsed.as_micros());
            elapsed
        };

        accumulator += {
            let t0 = Instant::now();

            let mut swapchain = Swapchain::new(1);
            let framebuffers = swapchain.framebuffers.clone();
            let working_framebuffer = swapchain.working_framebuffer.clone();
            let swapped = swapchain.swapped.clone();

            let render_thread = thread::spawn(move || {
                loop {
                    let mut current_fb = working_framebuffer.lock().unwrap();
                    current_fb = swapped.wait(current_fb).unwrap();

                    let index = *current_fb;
                    framebuffers[index].lock().unwrap().flush().unwrap();
                }
            });

            let mut avg = Duration::ZERO;
            let mut iteration = 0;
            let dt = Duration::from_secs_f64(1.0 / 10000.0);

            let mut avg_frame_time = Duration::ZERO;
            let mut avg_wait_time = Duration::ZERO;

            let mut start_time = Instant::now();
            while t0.elapsed() < Duration::from_secs(5) {
                let elapsed_time = start_time.elapsed();
                start_time = Instant::now();
                avg_frame_time += elapsed_time;

                let t0_avg = Instant::now();
                swapchain.queue_commands(&command_buffer);
                swapchain.swap();

                if dt > start_time.elapsed() {
                    avg_frame_time += dt - start_time.elapsed();
                    thread::sleep(dt - start_time.elapsed());
                }

                avg += t0_avg.elapsed();
                iteration += 1;
            }

            render_thread.join().unwrap();

            avg_frame_time = avg_frame_time / iteration;
            avg_wait_time = avg_wait_time / iteration;

            avg = avg / iteration;
            let _ = writer.into_inner().unwrap().into_raw_fd();
            debug!("Command Buffer Multiple Frames Avg Time: {} micro-seconds", avg.as_micros());
            debug!("Average frame time: {} micro-seconds", avg_frame_time.as_micros());
            debug!("Average wait time: {} milli-seconds", avg_wait_time.as_millis());
            avg
        };


        accumulator
    }

    pub fn run() {
        debug!("Starting crossterm benchmark");
        let mut accumulator = Duration::ZERO; 
        accumulator += command_buffer_usage();

        debug!("Total average time taken for benchmark: {} micro-seconds", accumulator.as_micros());
    }
}
