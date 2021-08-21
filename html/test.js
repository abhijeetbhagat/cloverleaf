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

const localVideo = document.getElementById('localVideo');
const remoteVideo = document.getElementById('remoteVideo');

let localstream;
let peer;

async function start() {
	console.log('Requesting local stream');
	startButton.disabled = true;
	try {
		const response = await fetch("http://localhost:8002/");
		const offer = await response.json();
		console.log(`got offer ${offer}`);
	} catch (e) {
		alert(`getUserMedia() error: ${e.name}`);
	}
}

async function sendAnswer(offer) {
	peer = new RTCPeerConnection({});
	await peer.setRemoteDescription(offer);

	const answer = await peer.createAnswer();
	await peer.setLocalDescription(answer);

	// TODO abhi: send this answer to remote peer via the /answer api

	const response = await fetch("http://localhost:8002/answer", {
		method: 'POST',
		body: {"type": "answer", "sdp": answer},
		headers: {
			"Content-Type": "application/json"
		}
	});
	const apiResponse = await response.json();
	console.log(`api response is ${apiResponse}`);
}
