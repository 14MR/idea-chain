async function sendSignature(message, signature) {

    var myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/json");

    const res = await fetch('http://localhost:8000/auth', {
        method: 'POST',
        mode: 'no-cors',
        headers: myHeaders,
        redirect: 'follow',
        body: JSON.stringify({
            message,
            signature
        })
    })

    return res.json()
}
export {sendSignature};