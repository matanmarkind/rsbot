This crate is where we actually create a bot to play OSRS. The 3 main structs we use for control are:

1. Capturer - used to take screenshots.

1. Framehandler - used to derive meaning from screenshots.

1. InputBot - used to feed in commands to the computer, emulates a keyboard and
mouse as well as logic for movig the mouse to a destination.

# Actions

We define a library of Actions for the bot to take. These are common things done by multiple scripts. We define them using a trait so that they can be composed together. For instance ConsumeInventory takes in a Vec of Actions to perform in order to fill up the inventory.

# Random Events

There is no support for this currently and I don't know what I would do to add support for this. My approach is to try to emulate an actual player, so I am not reading at a programatic level what is happening as opposed to some bots. In order to recognize a random event I think I would need to be able to read arbitrary text on the screen to see if my name is being mentioned. I don't know how I would do that without much more advanced ComputerVision algorithms, likely a canned NeuralNet.

Rust's support for tensorflow is a bit lacking at the moment, so not planning to pursue this now.

# Login

The bot currently doesn't handle logging in or changing worlds.