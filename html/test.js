/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree.
 */

'use strict';

const startButton = document.getElementById('startButton');
const callButton = document.getElementById('callButton');
const hangupButton = document.getElementById('hangupButton');
callButton.disabled = true;
hangupButton.disabled = true;
startButton.addEventListener('click', start);

const remoteVideo = document.getElementById('remoteVideo');

let localstream;
let peer;

async function start() {
	startButton.disabled = true;
	try {
		const response = await fetch("http://localhost:8888/");
		const offer = await response.json();
		console.log(`got offer ${offer}`);
		await sendAnswer(offer);
	} catch (e) {
		alert(`start() error: ${e.name}`);
	}
}

async function sendAnswer(offer) {
	peer = new RTCPeerConnection({});
	peer.addEventListener("icecandidate", e => onIceCandidate(peer, e));
	peer.addEventListener("iceconnectionstatechange", e => onIceStateChange(peer, e));
	peer.addEventListener("track", gotRemoteStream);

	await peer.setRemoteDescription(offer);

	const answer = await peer.createAnswer();
	await peer.setLocalDescription(answer);
	console.log(`going to send this answer: ${JSON.stringify(answer)}`);

	const response = await fetch("http://localhost:8888/answer", {
		method: 'POST',
		body: {"type": "answer", "sdp": JSON.stringify(answer)},
		headers: {
			"Content-Type": "application/json"
		}
	});
	const apiResponse = await response.json();
	console.log(`api response is ${apiResponse}`);
}

async function gotRemoteStream(e) {
	remoteVideo.srcObject = e.streams[0];
}

async function onIceCandidate(peer, e) {
	try {
		await peer.addIceCandidate(event.candidate);
		console.log("candidate added");
		const response = await fetch("http://localhost:8888/candidate", {
		method: 'POST',
		body: {"type": "candidate", "candidate": event.candidate},
		headers: {
			"Content-Type": "application/json"
		}});
	} catch (e) {
		console.log("error setting ice candidate");
	}
}

async function onIceStateChange(peer, e) {
	console.log("ice state changed");
}
