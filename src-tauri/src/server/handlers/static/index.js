let id;
let ws;
let pc;
let stream;
let _state;

const buttonStreamText = "Stream me baby";
const buttonStopStremText = "Stop me now";
Object.defineProperty(this, "state", {
  set: function(val) {
    _state = val;

    // change button behavior if is not in connected state
    const buttonEl = document.querySelector("#stream_button");
    if (buttonEl) {
      buttonEl.innerHTML = val !== "connected" ? buttonStopStremText : buttonStreamText;
      buttonEl.onclick = val !== "connected" ? handleButtonStopStream : handleButtonStream;
    }
  },
  get: function() {
    return _state;
  }
})

async function handleAnswer(answer) {
  if (state !== "awaiting_answer") {
    return;
  };

  pc.setRemoteDescription(new RTCSessionDescription(JSON.parse(answer)));

  state = "awaiting_candidate";
}

async function createOffer() {
  if (state !== "connected") {
    return;
  };

  const offer = await pc.createOffer();
  await pc.setLocalDescription(offer);

  const msg = {
    cmd: "offer",
    payload: offer,
    id,
  }

  ws.send(JSON.stringify(msg));

  state = "awaiting_answer";
}

async function handleCandidate(candidate) {
  if (state !== "awaiting_candidate") {
    return;
  }

  await pc.addIceCandidate(new RTCIceCandidate(JSON.parse(candidate)));
}

async function handleButtonStream() {
  const accepted = await prepareScreenSharing();
  if (accepted) {
    await createOffer();
  }
}

async function handleButtonStopStream() {
  cleanUp();
  createPeerConnection();
}

async function prepareScreenSharing() {
  stream = await navigator.mediaDevices.getDisplayMedia({
    audio: false,
    video: {
      displaySurface: "monitor"
    }
  });

  if (!stream) { return false };

  stream.getTracks().forEach(track => {
    // listen for clicks on stop sharing button and reset connection
    track.addEventListener("ended", () => {
      cleanUp();
      createPeerConnection();
    });
    pc.addTrack(track, stream);
  })

  return true;
}

function createPeerConnection() {
  pc = new RTCPeerConnection();
  pc.createDataChannel("data");
}


function cleanUp() {
  pc.close();
  state = "connected";

  // clean up streams
  stopTracks();

  // send disconnect
  const msg = {
    cmd: "disconnect",
    payload: "",
    id,
  }

  ws.send(JSON.stringify(msg));
}

function stopTracks() {
  for (let track of stream.getTracks()) {
    track.stop();
  }
}

function handleAlive(aliveId) {
  if (JSON.parse(aliveId) !== String(id)) {
    cleanUp();
    createPeerConnection();
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  id = Math.random().toString().substring(2);

  createPeerConnection();

  const host = window.location.hostname;
  ws = new WebSocket(`wss://${host}:443/api`);

  ws.onopen = () => {
    const msg = {
      cmd: "connect",
      payload: "",
      id,
    }

    ws.send(JSON.stringify(msg));

    state = "connected";
  };

  ws.onclose = () => {
    console.log("WebSocket closed");
  };

  ws.onmessage = (event) => {
    try {
      const parsed = JSON.parse(event.data);

      switch (parsed.cmd) {
        case "answer":
          handleAnswer(parsed.payload);
          break

        case "candidate":
          handleCandidate(parsed.payload);
          break

        case "alive":
          handleAlive(parsed.payload);
          break

        default:
          break
      }
    } catch (_) { }
  };
})
