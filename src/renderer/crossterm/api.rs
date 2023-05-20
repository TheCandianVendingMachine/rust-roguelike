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
    use std::io::BufWriter;
    use std::os::unix::io::{ FromRawFd, IntoRawFd };
    use std::fs::File;
    use std::thread;
    use log::debug;
    use std::time::{Instant, Duration};
    use crate::renderer::colours::Colour;
    use crate::renderer::crossterm::{ 
        colour,
        device_settings::DeviceSettings,
        render_engine::RenderEngine,
        command_buffer::CommandBuffer, command::{ Command, ClearInfo, DrawInfo }
    };

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


            let mut avg = Duration::ZERO;
            let mut iteration = 0;
            let dt = Duration::from_secs_f64(1.0 / 60.0);

            let mut avg_frame_time = Duration::ZERO;
            let mut avg_wait_time = Duration::ZERO;

            let mut start_time = Instant::now();
            while t0.elapsed() < Duration::from_secs(3) {
                let elapsed_time = start_time.elapsed();
                start_time = Instant::now();
                avg_frame_time += elapsed_time;
                if elapsed_time <= dt {
                    thread::sleep(dt - elapsed_time);
                    avg_wait_time += dt - elapsed_time;
                }


                let t0_avg = Instant::now();
                // todo: implement swapchain so we can output in different thread
                command_buffer.execute(&mut writer);
                avg += t0_avg.elapsed();
                iteration += 1;
            }

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
