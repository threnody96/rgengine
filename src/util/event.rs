use ::util::{ context };
use ggez::gilrs;
use ggez::winit::{self, dpi};
use ggez::input::gamepad::{ GamepadId };
use ggez::input::keyboard::{ KeyCode, KeyMods };
use ggez::event::{ EventHandler, EventsLoop, MouseButton, Axis, Button };
use ggez::event::winit_event::*;
use ggez::{ Context, GameResult };

pub fn quit() {
    context(|ctx| {
        ctx.continuing = false;
    });
}

pub fn run<S>(events_loop: &mut EventsLoop, state: &mut S) -> GameResult
    where
        S: EventHandler,
{
    use ggez::input::{keyboard, mouse};

    while { context(|ctx| ctx.continuing ) } {
        // If you are writing your own event loop, make sure
        // you include `timer_context.tick()` and
        // `ctx.process_event()` calls.  These update ggez's
        // internal state however necessary.
        { context(|ctx| ctx.timer_context.tick()); }
        events_loop.poll_events(|event| {
            { context(|ctx| ctx.process_event(&event)); }
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        // let actual_size = logical_size;
                        state.resize_event(
                            logical_size.width as f32,
                            logical_size.height as f32,
                        );
                    }
                    WindowEvent::CloseRequested => {
                        if !state.quit_event() {
                            quit();
                        }
                    }
                    WindowEvent::Focused(gained) => {
                        state.focus_event( gained);
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        state.text_input_event(ch);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            modifiers,
                            ..
                        },
                        ..
                    } => {
                        let repeat = context(|ctx| keyboard::is_key_repeated(ctx));
                        state.key_down_event(keycode, modifiers.into(), repeat);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(keycode),
                            modifiers,
                            ..
                        },
                        ..
                    } => {
                        state.key_up_event(keycode, modifiers.into());
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => {
                                (x as f32, y as f32)
                            }
                        };
                        state.mouse_wheel_event( x, y);
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
                        let position = { context(|ctx| mouse::position(ctx)) };
                        match element_state {
                            ElementState::Pressed => {
                                state.mouse_button_down_event(button, position.x, position.y)
                            }
                            ElementState::Released => {
                                state.mouse_button_up_event(button, position.x, position.y)
                            }
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let (position, delta) = context(|ctx| {
                            (mouse::position(ctx), mouse::delta(ctx))
                        });
                        state.mouse_motion_event(position.x, position.y, delta.x, delta.y);
                    }
                    _x => {
                        // trace!("ignoring window event {:?}", x);
                    }
                },
                Event::DeviceEvent { event, .. } => match event {
                    _ => (),
                },
                Event::Awakened => (),
                Event::Suspended(_) => (),
            }
        });
        // Handle gamepad events if necessary.
        if { context(|ctx| ctx.conf.modules.gamepad) } {
            while let Some(gilrs::Event { id, event, .. }) = { context(|ctx| ctx.gamepad_context.next_event()) } {
                match event {
                    gilrs::EventType::ButtonPressed(button, _) => {
                        state.gamepad_button_down_event(button, GamepadId(id));
                    }
                    gilrs::EventType::ButtonReleased(button, _) => {
                        state.gamepad_button_up_event(button, GamepadId(id));
                    }
                    gilrs::EventType::AxisChanged(axis, value, _) => {
                        state.gamepad_axis_event(axis, value, GamepadId(id));
                    }
                    _ => {}
                }
            }
        }
        state.update()?;
        state.draw()?;
    }

    Ok(())
}