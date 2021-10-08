import { useRef } from 'react';
import React from 'react';

function App() {
    const [startDisable, setStartDisable] = React.useState(false);
    const [callDisable, setCallDisable] = React.useState(true);
    const [hangUpDisable, setHangUpDisable] = React.useState(true);

    const clientVideo = useRef(null);

    let localstream;
    let peer;
    let session;

    async function start() {
        setStartDisable(true);
        try {
            const response = await fetch("http://localhost:8888/");
            const offer = await response.json();
            this.session = offer.session;
            console.log(`got offer ${offer.sdp}`);
            await sendAnswer(offer);
        } catch (e) {
            console.error(e);
        }
    }

    async function sendAnswer(offer) {
        this.peer = new RTCPeerConnection({});
        this.peer.addEventListener("icecandidate", e => onIceCandidate(this.peer, e));
        this.peer.addEventListener("iceconnectionstatechange", e => onIceStateChange(this.peer, e));
        this.peer.addEventListener("track", gotRemoteStream);
    
        // even though the offer JSON object has user-defined fields along with the remote sdp,
        // setRemoteDescription will correctly extract the sdp
        await peer.setRemoteDescription(offer);
    
        const answer = await peer.createAnswer();
        await peer.setLocalDescription(answer);
        console.log(`going to send this answer: ${JSON.stringify(answer)}`);
    
        const response = await fetch("http://localhost:8888/answer", {
        method: 'POST',
        body: JSON.stringify({ "pt": "Answer", "payload": JSON.stringify(answer), "id": "", "session": offer.session }),
        headers: {
            "Content-Type": "application/json"
        }
        });
        const apiResponse = await response.json();
        console.log(`api response is ${apiResponse}`);
    }

    async function gotRemoteStream(e) {
        this.clientVideo.current.srcObject = e.streams[0];
    }

    async function onIceCandidate(peer, event) {
        try {
        if(event.candidate) {
            // await peer.addIceCandidate(event.candidate);
            const response = await fetch("http://localhost:8888/candidate", {
                method: 'POST',
                body: JSON.stringify({ "pt": "Candidate", "payload": JSON.stringify(event.candidate), "id": "", "session": this.session }),
                headers: {
                    "Content-Type": "application/json"
                }
            });
    
            const apiResponse = await response.json();
            console.log(`api response is ${apiResponse}`);
        } else {
            console.log('we are done gathering candidates');
            const response = await fetch("http://localhost:8888/watch", {
                method: 'POST',
                body: JSON.stringify({ "pt": "Candidate", "payload": JSON.stringify(event.candidate), "id": "", "session": this.session }),
                headers: {
                    "Content-Type": "application/json"
                }
            });
    
            const apiResponse = await response.json();
            console.log(`api response is ${apiResponse}`);
        }
        } catch (e) {
            console.log("error setting ice candidate");
        }
    }
    
    async function onIceStateChange(peer, e) {
        console.log("ice state changed");
    }

    return (
        <>
            <video ref={clientVideo} playsInline autoPlay></video>
            <div className="box">
                <button disabled={startDisable} onClick={(event) => start()}>Start</button>
                <button disabled={callDisable}>Call</button>
                <button disabled={hangUpDisable}>Hang Up</button>
            </div>
        </>
    );
}

export default App;