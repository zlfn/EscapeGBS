use std::fs::File;
use std::io::prelude::*;
use rocket::response::content;
use std::{fs, io};
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;
use std::convert::From;
use super::GameState;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct choice
{
    text:String,
    link:String,
    req:u32,
    number:usize
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JsonFormat
{
    comment:String,
    location:String,
    image:String,
    conditional:bool,
    choices:Vec<choice>
}

///입력된 주소에서 파일을 찾아 읽어서 String으로 반환합니다.
///
/// # Example
/// ```
/// let mut s = DataManager::FileIO::ReadFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadFile(path: &str) -> Result<String, io::Error>
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(String::from(contents));
}

///입력된 주소에서 HTML파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadHTMLFile(path: &str) -> Result<content::Html<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::Html(s));
}

///입력된 주소에서 CSS 파일을 찾아 읽어서 content::Css로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadCSSFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadCSSFile(path: &str) -> Result<content::Css<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::Css(s));
}

///입력된 주소에서 JS 파일을 찾아 읽어서 content::JavaScript로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadJSFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadJSFile(path: &str) -> Result<content::JavaScript<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::JavaScript(s));
}

pub fn BuildHTML_nostate(to:i32) -> Result<content::Html<String>,io::Error>
{
    let mut json_path = format!("JSON/{}.json", to);
    let mut file = &ReadFile(&json_path).unwrap()[..];

    let mut data: JsonFormat = serde_json::from_str(file).unwrap();


    let mut s = format!(
        "<!DOCTYPE HTML>\n\
        <head>\n\
            <style>\n\
            @import url(//fonts.googleapis.com/earlyaccess/jejumyeongjo.css);\n\
            @font-face {{
            font-family: '국립박물관문화재단클래식B';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_twelve@1.0/국립박물관문화재단클래식B.woff') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        @font-face {{
            font-family: 'IM_Hyemin-Regular';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_2106@1.1/IM_Hyemin-Regular.woff2') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        @font-face {{
            font-family: 'KOTRA_SONGEULSSI';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_20-10-21@1.0/KOTRA_SONGEULSSI.woff') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        #container{{
            width: 56.25vw;
            height: 100vw;
        }}\n\

        html{{
            overscroll-behavior: contain;
        }}
        @media(max-aspect-ratio:1/1)
        {{\n\
            .scroll_box {{
                overflow-y:hidden;
                overflow-x:hidden;
                scrollbar-width:none;
                -ms-overflow-style:none;
                position:relative;
                top:0px;
                margin-bottom:200px;
            }}\n\
            .scroll_box::-webkit-scrollbar
            {{
                display:none;
            }}\n\
            .mimage {{
                width:80%;
                text-align:center;
                display: block;
                margin-left:auto;
                margin-right:auto;
                position:relative;
                margin-bottom:5px;
            }}\n\
            .typing-text{{
                display: table-cell;
                vertical-align: middle;
                font-size: 18px;
                -ms-scrollbar-track-color: white;
                font-family: 'Jeju Myeongjo', serif;
                padding-top:10px;
                padding-left:10px;
                padding-right:10px;
                line-height:25px;
            }}\n\
            .location {{
                text-align:center;
                height:10%;
                position:sticky;
                top:0;
                font-family: '국립박물관문화재단클래식B';
                background-color:white;
                z-index:1;
            }}\n\
            .bottom {{
                position:fixed;
                overflow:hidden;
                bottom:0;
                width:100%;
                margin-top:200px;
                background-color: white;
                height:150px;
                z-index:2;
            }}\n\
            body{{
            margin:0;
            padding:0;
            overscroll-behavior: contain;
            overflow-y:scroll;
            scrollbar-width:none;
            -ms-overflow-style:none;
            animation: fadein 2000ms ease-out;
            -moz-animation: fadein 2000ms ease-out; /* Firefox */
            -webkit-animation: fadein 2000ms ease-out; /* Safari and  Chrome */
            -o-animation: fadein 2000ms ease-out; /* Opera */
        }}\n\
        }}\n\
       @media(min-aspect-ratio:1/1)
        {{\n\
            .scroll_box {{
                overflow-y:hidden;
                overflow-x:hidden;
            }}\n\
            .mimage_box {{
                width:50%;
                height:100%;
                overflow:hidden;
                float:left;
                margin-top:70px;
                margin-bottom:20px;
                padding-left:5px;
                z-index:2;
            }}\n\
            .mimage {{
                position:relative;
                margin:auto;
                width:100%;
                height:auto;
                object-fit:none;
                display:inline;
                transform: translate(50, 50);
                z-index:2;
            }}\n\
            .typing-text{{
                overflow-y:scroll;
                overflow-x:hidden;
                font-size: 18px;
                width:49%;
                font-family: 'Jeju Myeongjo', serif;
                line-height:25px;
                float:left;
                position:absolute;
                margin-left:50%;
                padding-left:10px;
                bottom:200px;
                top:120px;
                z-index:2;
            }}\n\
            .location {{
                text-align:center;
                height:30px;
                position:relative;
                top:0;
                font-family: '국립박물관문화재단클래식B';
                background-color:white;
                z-index:1;
            }}\n\
            .bottom {{
                position:fixed;
                overflow:hidden;
                width:100%;
                bottom:0;
                background-color: white;
                height:200px;
                z-index:2;
}}\n\

            body{{
            margin:0;
            padding:0;
            overscroll-behavior: contain;
            overflow-y:hidden;
            scrollbar-width:none;
            -ms-overflow-style:none;
            animation: fadein 2000ms ease-out;
            -moz-animation: fadein 2000ms ease-out; /* Firefox */
            -webkit-animation: fadein 2000ms ease-out; /* Safari and  Chrome */
            -o-animation: fadein 2000ms ease-out; /* Opera */
            }}\n\
        }}\n\

        .select_button {{
            font-family: 'Jeju Myeongjo',serif;
            font-size:15px;
            background-color:white;
            width:100%;
            height:50px;
            padding-bottom:3px;
            border-top:0px;
            border-bottom:1px solid black;
            border-top:1px solid black;
            transform: translate3d(0, 0, 0);
            -webkit-text-fill-color:black;
        }}\n\
        .select_scroll_box {{
            position:relative;
            overflow-y:scroll;
            overflow-x:hidden;
            scrollbar-width:none;
            -ms-overflow-style:none;
            height:96%;
            width:100%;
            bottom:0;
            z-index:1;
            margin-left:auto;
            margin-right:auto;
        }}\n\
        @keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-moz-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-webkit-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-o-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity: 1;}}
        }}\n\
        </style>
    <meta charset=\"utf-8\">
    <meta http-equiv=\"page-enter\" content=\"blendTrans(duration=0.3)\">
    <meta http-equiv=\"page-exit\" content=\"blendTrans(duration=0.3)\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
</head>\n\
<body>\n
<div class=\"location\">
    <hr>
    <h1>{}</h1>
    <hr>
</div>
    <div class=\"scroll_box\">
        <div class=\"mimage_box\">
    <img class=\"mimage\" src=\"{}\"/>
            </div>
    <p class=\"typing-text\"></p>
    </div>\n\
<div class=\"bottom\">
    <hr>
    <div class=\"select_scroll_box\">
        <form method=\"post\">
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
        </form>
    </div>
</div>\n

<script>
        const content = \"{}\";
        const text = document.querySelector(\".typing-text\");
        let index = 0;

        window.history.forward();
        function noBack(){{window.history.forward();}} //뒤로가기 방지

        function checkRow(event) {{ var event_test = event; alert(event_test); }}

        history.pushState(null, null, location.href);
        window.onpopstate = function(event) {{
            history.go(1);
        }};\n

        function typing() {{
            let txt = content[index++];
            if(index <= content.length) {{
                if(txt==\"@\") text.innerHTML += \"<br/>\";
                else if(txt==\"`\") text.innerHTML +=\"&nbsp&nbsp\";
                else text.innerHTML += txt;
            }}
        }}

        setInterval(typing, 8);


        jQuery(function($) {{
            $(\"body\").css(\"display\", \"none\");
            $(\"body\").fadeIn(2000);
            $(\"a.transition\").click(function(event){{
                event.preventDefault();
                linkLocation = this.href;
                $(\"body\").fadeOut(1000, redirectPage);
            }});
            function redirectPage() {{
                window.location = linkLocation;
            }}
        }});


    </script>
</body>

",      data.location, data.image,
        data.choices[0].link,data.choices[0].text,
        data.choices[1].link,data.choices[1].text,
        data.choices[2].link,data.choices[2].text,
        data.choices[3].link,data.choices[3].text,
        data.choices[4].link,data.choices[4].text,
        data.choices[5].link,data.choices[5].text,
        data.comment);

    return Ok(content::Html(s));
}


pub fn BuildHTML(from:i32, to:i32, state:&GameState) ->Result<content::Html<String>,io::Error>
{
    let mut json_path = format!("JSON/{}.json", to);
    let mut file = &ReadFile(&json_path).unwrap()[..];

    let mut data: JsonFormat = serde_json::from_str(file).unwrap();

    for mut c in data.choices.iter_mut()
    {
        if c.req==0 {continue;}
        if (c.req==1) && (!state.condition[c.number])
        {
            c.text = String::from("-");
            c.link = String::from(format!("{} 0",from));
        }
        if (c.req==2) && (!state.gadget[c.number])
        {
            c.text = String::from("-");
            c.link = String::from(format!("{} 0",from));
        }
    }

    if data.conditional==true
    {
        for i in (1..5)
        {
            if data.choices[i-1].text==String::from("-")
            {
                data.choices.swap(i-1,i);
            }
        }
    }


    let mut s = format!(
        "<!DOCTYPE HTML>\n\
        <head>\n\
            <style>\n\
            @import url(//fonts.googleapis.com/earlyaccess/jejumyeongjo.css);\n\
            @font-face {{
            font-family: '국립박물관문화재단클래식B';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_twelve@1.0/국립박물관문화재단클래식B.woff') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        @font-face {{
            font-family: 'IM_Hyemin-Regular';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_2106@1.1/IM_Hyemin-Regular.woff2') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        @font-face {{
            font-family: 'KOTRA_SONGEULSSI';
            src: url('https://cdn.jsdelivr.net/gh/projectnoonnu/noonfonts_20-10-21@1.0/KOTRA_SONGEULSSI.woff') format('woff');
            font-weight: normal;
            font-style: normal;
        }}\n\
        #container{{
            width: 56.25vw;
            height: 100vw;
        }}\n\

        html{{
            overscroll-behavior: contain;
        }}
        @media(max-aspect-ratio:1/1)
        {{\n\
            .scroll_box {{
                overflow-y:hidden;
                overflow-x:hidden;
                scrollbar-width:none;
                -ms-overflow-style:none;
                position:relative;
                top:0px;
                margin-bottom:200px;
            }}\n\
            .scroll_box::-webkit-scrollbar
            {{
                display:none;
            }}\n\
            .mimage {{
                width:80%;
                text-align:center;
                display: block;
                margin-left:auto;
                margin-right:auto;
                position:relative;
                margin-bottom:5px;
            }}\n\
            .typing-text{{
                display: table-cell;
                vertical-align: middle;
                font-size: 18px;
                -ms-scrollbar-track-color: white;
                font-family: 'Jeju Myeongjo', serif;
                padding-top:10px;
                padding-left:10px;
                padding-right:10px;
                line-height:25px;
            }}\n\
            .location {{
                text-align:center;
                height:10%;
                position:sticky;
                top:0;
                font-family: '국립박물관문화재단클래식B';
                background-color:white;
                z-index:1;
            }}\n\
            .bottom {{
                position:fixed;
                overflow:hidden;
                bottom:0;
                width:100%;
                margin-top:200px;
                background-color: white;
                height:150px;
                z-index:2;
            }}\n\
            body{{
            margin:0;
            padding:0;
            overscroll-behavior: contain;
            overflow-y:scroll;
            scrollbar-width:none;
            -ms-overflow-style:none;
            animation: fadein 2000ms ease-out;
            -moz-animation: fadein 2000ms ease-out; /* Firefox */
            -webkit-animation: fadein 2000ms ease-out; /* Safari and  Chrome */
            -o-animation: fadein 2000ms ease-out; /* Opera */
        }}\n\
        }}\n\
       @media(min-aspect-ratio:1/1)
        {{\n\
            .scroll_box {{
                overflow-y:hidden;
                overflow-x:hidden;
            }}\n\
            .mimage_box {{
                width:50%;
                height:100%;
                overflow:hidden;
                float:left;
                margin-top:70px;
                margin-bottom:20px;
                padding-left:5px;
                z-index:2;
            }}\n\
            .mimage {{
                position:relative;
                margin:auto;
                width:100%;
                height:auto;
                object-fit:none;
                display:inline;
                transform: translate(50, 50);
                z-index:2;
            }}\n\
            .typing-text{{
                overflow-y:scroll;
                overflow-x:hidden;
                font-size: 18px;
                width:49%;
                font-family: 'Jeju Myeongjo', serif;
                line-height:25px;
                float:left;
                position:absolute;
                margin-left:50%;
                padding-left:10px;
                bottom:200px;
                top:120px;
                z-index:2;
            }}\n\
            .location {{
                text-align:center;
                height:30px;
                position:relative;
                top:0;
                font-family: '국립박물관문화재단클래식B';
                background-color:white;
                z-index:1;
            }}\n\
            .bottom {{
                position:fixed;
                overflow:hidden;
                width:100%;
                bottom:0;
                background-color: white;
                height:200px;
                z-index:2;
}}\n\

            body{{
            margin:0;
            padding:0;
            overscroll-behavior: contain;
            overflow-y:hidden;
            scrollbar-width:none;
            -ms-overflow-style:none;
            animation: fadein 2000ms ease-out;
            -moz-animation: fadein 2000ms ease-out; /* Firefox */
            -webkit-animation: fadein 2000ms ease-out; /* Safari and  Chrome */
            -o-animation: fadein 2000ms ease-out; /* Opera */
            }}\n\
        }}\n\

        .select_button {{
            font-family: 'Jeju Myeongjo',serif;
            font-size:15px;
            background-color:white;
            width:100%;
            height:50px;
            padding-bottom:3px;
            border-top:0px;
            border-bottom:1px solid black;
            border-top:1px solid black;
            transform: translate3d(0, 0, 0);
            -webkit-text-fill-color:black;
        }}\n\
        .select_scroll_box {{
            position:relative;
            overflow-y:scroll;
            overflow-x:hidden;
            scrollbar-width:none;
            -ms-overflow-style:none;
            height:96%;
            width:100%;
            bottom:0;
            z-index:1;
            margin-left:auto;
            margin-right:auto;
        }}\n\
        @keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-moz-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-webkit-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity:1;}}
        }}
        @-o-keyframes fadein {{
            from {{opacity:0;}}
            to {{opacity: 1;}}
        }}\n\
        </style>
    <meta charset=\"utf-8\">
    <meta http-equiv=\"page-enter\" content=\"blendTrans(duration=0.3)\">
    <meta http-equiv=\"page-exit\" content=\"blendTrans(duration=0.3)\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
</head>\n\
<body>\n
<div class=\"location\">
    <hr>
    <h1>{}</h1>
    <hr>
</div>
    <div class=\"scroll_box\">
        <div class=\"mimage_box\">
    <img class=\"mimage\" src=\"{}\"/>
            </div>
    <p class=\"typing-text\"></p>
    </div>\n\
<div class=\"bottom\">
    <hr>
    <div class=\"select_scroll_box\">
        <form method=\"post\">
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
            <button class=\"select_button\" type=\"submit\" value=\"{}\" name=\"selection\">{}</button>
        </form>
    </div>
</div>\n

<script>
        const content = \"{}\";
        const text = document.querySelector(\".typing-text\");
        let index = 0;

        window.history.forward();
        function noBack(){{window.history.forward();}} //뒤로가기 방지

        function checkRow(event) {{ var event_test = event; alert(event_test); }}

        history.pushState(null, null, location.href);
        window.onpopstate = function(event) {{
            history.go(1);
        }};\n

        function typing() {{
            let txt = content[index++];
            if(index <= content.length) {{
                if(txt==\"@\") text.innerHTML += \"<br/>\";
                else if(txt==\"`\") text.innerHTML +=\"&nbsp&nbsp\";
                else text.innerHTML += txt;
            }}
        }}

        setInterval(typing, 8);


        jQuery(function($) {{
            $(\"body\").css(\"display\", \"none\");
            $(\"body\").fadeIn(2000);
            $(\"a.transition\").click(function(event){{
                event.preventDefault();
                linkLocation = this.href;
                $(\"body\").fadeOut(1000, redirectPage);
            }});
            function redirectPage() {{
                window.location = linkLocation;
            }}
        }});


    </script>
</body>

",      data.location, data.image,
        data.choices[0].link,data.choices[0].text,
        data.choices[1].link,data.choices[1].text,
        data.choices[2].link,data.choices[2].text,
        data.choices[3].link,data.choices[3].text,
        data.choices[4].link,data.choices[4].text,
        data.choices[5].link,data.choices[5].text,
        data.comment);

    return Ok(content::Html(s));
}
