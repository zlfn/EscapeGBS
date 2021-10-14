const content = "\a눈을 뜨고 시간을 확인했는데 9시 반이다.\n 어?지각이다.. 룸메이트는 날 왜 안 깨운 것일까. 너무 깊이 잔 것일까 생각하다 보니, 문득 어제 방학을 했다는 사실이 생각났다..... 나 어제 방학했구나... 방학이 시작한 걸 까먹고 있었다. \n\n\a 근데 방학인데 나는 왜 기숙사일까? 그렇다. 나는 어제 방학식 이후 귀사했어야 하나 잠이 들어버렸다. \n\n\a 그럼, 이제 어떻게 해야 할까?"
const text = document.querySelector(".typing-text")
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

