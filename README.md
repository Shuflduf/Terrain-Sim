# Terrain Simulator
Terrain generator and renderer using WebGPU

# Features
- Free camera controls
- Normal based lighting
- Animated transparent water
- [Skybox](https://freestylized.com/skybox/sky_05/)
- VSync toggle
- Frustum culling
- FPS tracker
- FULL cross-platform support

# Installation
While it's recommened to install Terrain Simulator natively for maximum performance and system integration, you can experience a demo in your web browser [here](https://shuflduf.github.io/Terrain-Simulator/)!

## Cargo (Recommended)

1. `cargo install terrain-sim`
2. Execute with `terrain-sim` anywhere

## Linux
1. Go to the [latest release](https://github.com/Shuflduf/Terrain-Simulator/releases/latest)
2. Download the binary for your architecture:
	- `terrain-sim-linux-x86_64` (Intel/AMD 64-bit)
	- `terrain-sim-linux-arm64` (ARM 64-bit)
	- `terrain-sim-linux-armv7` (ARM 32-bit)
3. Make it executable: `chmod +x terrain-sim-linux-[ARCH]`
4. **Option A: Add to PATH (recommended)**
	```bash
	sudo mv terrain-sim-linux-[ARCH] /usr/local/bin/terrain-sim

	# now you can run terrain-sim from anywhere
	terrain-sim
	```
5. **Option B: Run from current directory**
	```bash
	mv terrain-sim-linux-[ARCH] terrain-sim
	./terrain-sim
	```

## MacOS
1. Go to the [latest release](https://github.com/Shuflduf/Terrain-Simulator/releases/latest)
2. Download the binary for your architecture:
	- `terrain-sim-macos-x86_64` (Intel Macs)
	- `terrain-sim-macos-arm64` (Apple Silicon)
3. Make it executable: `chmod +x terrain-sim-macos-[ARCH]`
4. **Option A: Add to PATH (recommended)**
	```bash
	sudo mv terrain-sim-macos-[ARCH] /usr/local/bin/terrain-sim

	# now you can run terrain-sim from anywhere
	terrain-sim
	```
5. **Option B: Run from current directory**
	```bash
	mv terrain-sim-macos-[ARCH] terrain-sim
	./terrain-sim
	```

## Windows
1. Go to the [latest release](https://github.com/Shuflduf/Terrain-Simulator/releases/latest)
2. Download the `.exe` for your architecture:
	- `terrain-sim-windows-x86_64.exe` (Intel/AMD 64-bit)
	- `terrain-sim-windows-arm64.exe` (ARM 64-bit)
3. **Option A: Add to PATH (recommended)**
	- Move `terrain-sim-windows-[ARCH].exe` to `C:\Users\YourUsername\AppData\Local\Programs\terrain-sim\` (create the folder if it doesn't exist)
	- Open "Edit the system environment variables" and add the folder to your PATH.
	- Restart your terminal and now you can run terrain-sim from anywhere!
	```powershell
	terrain-sim
	```
4. **Option B: Run from current directory**
	```powershell
	ren "terrain-sim-windows-[ARCH].exe" "terrain-sim.exe"

	terrain-sim.exe
	```
