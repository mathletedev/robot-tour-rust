dev:
	cargo build --release --target thumbv7em-none-eabihf
	arm-none-eabi-objcopy -I binary -O ihex ./target/thumbv7em-none-eabihf/release/robot-tour robot-tour.hex
	cp robot-tour.hex /run/media/$(USER)/MICROBIT
