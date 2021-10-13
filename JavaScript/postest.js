const content = "나태양죽나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어나태양죽어"
const text = document.querySelector(".typing-text");
let index = 0;

window.history.forward();
function noBack(){window.history.forward();}

function checkRow(event) { var event_test = event; alert(event_test); }

function typing(){
    let txt = content[index++];
    if(index <= content.length){
        if(txt=="\n") text.innerHTML += "<br/>";
        else if(txt=="\a") text.innerHTML +="&nbsp&nbsp";
        else text.innerHTML += txt;
    }
}

setInterval(typing, 13)


jQuery(function($) {
    $("body").css("display", "none");
    $("body").fadeIn(2000);
    $("a.transition").click(function(event){
        event.preventDefault();
        linkLocation = this.href;
        $("body").fadeOut(1000, redirectPage);
    });
    function redirectPage() {
        window.location = linkLocation;
    }
});



