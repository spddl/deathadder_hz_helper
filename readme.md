# deathadder_hz_helper

[![Downloads][1]][2] [![GitHub stars][3]][4]

[1]: https://img.shields.io/github/downloads/spddl/deathadder_hz_helper/total.svg
[2]: https://github.com/spddl/deathadder_hz_helper/releases "Downloads"
[3]: https://img.shields.io/github/stars/spddl/deathadder_hz_helper.svg
[4]: https://github.com/spddl/deathadder_hz_helper/stargazers "GitHub stars"

The program allows you to change the Hz of a Razer DeathAdder V3 Pro in Windows without additional programs.

The project https://github.com/marcospb19/dawctl helped a lot

```ini
USAGE:
  deathadder_hz_helper.exe [OPTIONS]

CHANGE THE DEVICE (OPTIONAL):
  --VID NUMBER [default: 0x1532]
  --PID_WIRE NUMBER [default: 0x00c2]
  --PID_WIRELESS NUMBER [default: 0x00c3]
  --USAGE NUMBER [default: 2]
  --USAGEPAGE NUMBER [default: 1]

OPTIONS:
  --hz NUMBER       Sets the Hz number for cable-connected
  --whz NUMBER      Sets the Hz number for wireless
```
