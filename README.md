# Level
*Tim Pup*  

<br>
Bubble level using the IMU on the Microbit V2

---

The specifications:

- [x] When the board is upside down ( is positive), the display should be blanked.

- [x] Otherwise, a single LED on the display should be lit to show the "level point". Start in "coarse" mode: divide the range from -500 to 500 mG into 5 parts. The on LED should be at the position given by the and coordinates using this scaling. If the LED would be "off the edge" on either axis, clamp it to that axis.

- [x] The lit LED should be toward the higher edge of the board along the and axes; the board can be leveled by lowering the edges to bring the lit LED toward the center. When the on LED is in the center, the and components of acceleration are close to 0 mG, so the MB2 is roughly level.

- [x] Pressing the B button (by itself) should put the level in "fine" mode: now the LED scales should go from -50 to 50 mG. Pressing the A button (by itself) should return to "coarse" mode.

- [x] The level measurement and display should refresh every 200 ms (5 frames per second).

---


Motion sensor [Doc](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.21/MicroBit_V2.2.1_nRF52820%20schematic.PDF)

![Motion sensor schematic](./imgs/MotionSensor.png)

https://tech.microbit.org/hardware/schematic/
`P0.08 	I2C_INT_SCL`
`P0.16 	I2C_INT_SDA`