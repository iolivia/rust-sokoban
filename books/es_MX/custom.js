function addCodeLinkClass() {
    var elements = document.getElementsByTagName("blockquote");
    for (var i = 0; i < elements.length; i += 1) {
        var element = elements.item(i);
        if (element.innerHTML.includes("CODELINK")) {
            element.className = "code-link";
        }
    }
}

addCodeLinkClass();