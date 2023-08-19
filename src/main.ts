import { invoke } from "@tauri-apps/api/tauri";

let id: string;
let ws: WebSocket;
let pc: RTCPeerConnection;

async function handleOffer(com: any) {
  if (pc.connectionState === "connected") {
    pc.close()

    const aliveMsg = {
      cmd: "alive",
      payload: com.id,
      id,
    }

    ws.send(JSON.stringify(aliveMsg));

    createPeerConnection();
  }

  const offer = com.payload;

  pc.setRemoteDescription(new RTCSessionDescription(JSON.parse(offer)));

  const answer = await pc.createAnswer();
  await pc.setLocalDescription(answer);

  const msg = {
    cmd: "answer",
    payload: answer,
    id,
  }

  ws.send(JSON.stringify(msg));
}

async function handleDisconnect() {
  const videoEl: HTMLVideoElement | null = document.querySelector("#video_container");
  if (videoEl) {
    videoEl.srcObject = null;
  }
}

async function handleIceCandidateChange(event: RTCPeerConnectionIceEvent) {
  const candidate = event.candidate;

  if (candidate) {
    const msg = {
      cmd: "candidate",
      payload: candidate,
      id,
    };

    ws.send(JSON.stringify(msg));
  }
}

async function handleTrackChange(event: RTCTrackEvent) {
  const videoEl: HTMLVideoElement | null = document.querySelector("#video_container");
  if (videoEl) {
    const [remoteStream] = event.streams;
    videoEl.srcObject = remoteStream;
  }
}

function createPeerConnection() {
  pc = new RTCPeerConnection();
  pc.createDataChannel("data");

  pc.addEventListener("icecandidate", handleIceCandidateChange);
  pc.addEventListener("track", handleTrackChange);
}

async function displayIpAddress(): Promise<void> {
  const ipAddressEl = document.querySelector("#ip_address");
  const ip = await invoke<string>("find_my_ip", {});

  if (ipAddressEl && ip) {
    ipAddressEl.innerHTML += ip;
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  await displayIpAddress();

  id = Math.random().toString().substring(2);

  ws = new WebSocket(`ws:/localhost/api`);

  createPeerConnection();

  ws.onopen = () => {
    const msg = {
      cmd: "connect",
      payload: "",
      id,
    }

    ws.send(JSON.stringify(msg));
  };

  ws.onmessage = (event) => {
    try {
      const parsed = JSON.parse(event.data);

      switch (parsed.cmd) {
        case "offer":
          handleOffer(parsed);
          break

        case "disconnect":
          handleDisconnect();
          break

        default:
          break
      }
    } catch (_) { }
  };

});
