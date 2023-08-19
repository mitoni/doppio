# Doppio

Super simple and intutive screen sharing app for local networks.

## How it works

Uses Tauri with an Actix server that manages communications via websockets and WebRTC to display the screen.

## How to use

1. Install the app on a computer where you want to display the screen.
2. Connect to the same wifi or wired network as the dispay computer.
3. Whoever wants to share its screen connects to the displayed IP address using https.
4. Click on the button in the center of the screen and pick the screen you want to share. It defaults to entire screen.
5. When someone else wants to share their screen, connects to the same IP address and, after clicking on the button, stops the other connection and starts theirs. The other user get disconnected.
