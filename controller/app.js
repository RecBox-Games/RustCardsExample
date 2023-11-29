import { sendControlpadMessage } from "./controlpad.js"



// receive messages
document.addEventListener("controlpad-message", (msg) => {
    console.log("recv: " + msg);
});


function deal() {
    sendControlpadMessage("deal");
}

setInterval(deal, 1000);
