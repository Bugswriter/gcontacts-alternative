let query = document.getElementById("query")
let result = document.getElementById("result")
let btn = document.getElementById("btn")

const copyContent = async (event) => {
    // KISS function to just copy some content
    try {
	// phone = this.innerHTML
	phone = event.srcElement.innerHTML
	await navigator.clipboard.writeText(phone);
	alert("Number copied to clipboard");	
    } catch (err) {
	console.error('Failed to copy: ', err);
    }
}

async function fetch_results(){
    // fetch results from database
    var q = query.value
    if (q.length < 3){
	return
    }
    var res = await fetch("/search?q=" + q)
    var data = await res.json()
    result.innerHTML = ""
    for(var i=0; i<data.length; i++){
	// result.innerHTML += "<li>" + data[i]['name'] + ", " + data[i]['phone'] + "</li>"
	result.innerHTML += `<li>${data[i]['name']},<span onclick='copyContent(event)'>${data[i]['phone']}</span></li>`

    }
}
btn.onclick = fetch_results // attaching an event function to the button
query.onkeyup = fetch_results // on key up


