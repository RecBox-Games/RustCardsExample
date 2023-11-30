import { sendControlpadMessage } from "./controlpad.js"
import { showJoinBox } from "./join.js"
import { hideWaitBox } from "./wait.js"

                          
// receive messages
document.addEventListener("controlpad-message", (event) => {
    var msg = event.detail;
    console.log("recv: " + msg);
    var parts = msg.split(":");
    // TODO: should check that parts[0] is 'state'
    var state = parts[1];
    var arg1 = parts[2];
    var arg2 = parts[3];
    var arg3 = parts[4];
    if (state == "joining") {
        showJoinBox();
    } else if (state == "playing") {
        updatePlayingState(arg1, arg2, arg3);
        hideWaitBox();
    }
});


// must implement this function (called by controlpads.js)
export function controlpadStart() {
    // start by getting our current state since it's very possible we're
    // reconnecting and not just connecting for the first time
    sendControlpadMessage("state-request");
}

// must implement this function (called by controlpads.js)
// called 30 times per second
export function controlpadUpdate() {
    
}


function updatePlayingState(name, left_card_str, right_card_str) {
    // name
    showName(name);
    // size the card div
    let vw = window.innerWidth;
    let vh = window.innerHeight;
    // determine the shorter dimensions to account for portrait vs landscape
    let constraint = vw < vh ? "vw" : "vh";
    let card_div = document.getElementById("cardDiv");
    card_div.style.width = "100" + constraint;
    card_div.style.height = "58" + constraint;
    // left card
    if (left_card_str != "") {
        var card_img_left = createCardElement(left_card_str);
        card_img_left.style.left = "0%";
        card_div.appendChild(card_img_left);
    }
    // right card
    if (right_card_str != "") {
        var card_img_right = createCardElement(right_card_str);
        card_img_right.style.right = "0%";
        card_div.appendChild(card_img_right);
    }
    // deal button
    var deal_button = createDealButton();
    card_div.appendChild(deal_button);
}


function createDealButton() {
    var img = document.createElement("img");
    // load image
    img.src = "./resources/deal.png";
    // todo other things below
    img.addEventListener("click", function() {
        console.log("deal clicked");
    });
    // position the card
    img.style.position = "absolute";
    img.style.width ="40%";
    img.style.height = "35%";
    img.style.top = "105%";
    img.style.left = "50%";
    img.style.transform = "translate(-50%, 0%)";
    return img;
}

function createCardElement(card_str) {
    // parse card spec
    var parts = card_str.split(",");
    var suit = parts[0];
    var rank = parts[1];
    var img = document.createElement("img");
    // load image
    img.src = "./resources/card_fronts/card_" + suit + "_" + rank + ".png";
    // todo other things below
    img.addEventListener("click", function() {
        console.log("card clicked");
    });
    // position the card
    img.style.position = "absolute";
    img.style.width ="53%";
    img.style.height = "100%";
    img.style.top = "50%";
    img.style.transform = "translate(0%, -50%)";
    return img;
}

function showName(name) {
    let name_box = document.getElementById("nameBox");
    let name_line = document.getElementById("nameLine");
    name_line.textContent = name;
    name_box.style.display = "block";
}
