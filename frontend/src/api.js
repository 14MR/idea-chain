import wretch from 'wretch';

async function sendSignature(message, signature) {
    await wretch()
        .url(
            'http://localhost:8000/auth'
        )
        .content('application/json')
        // .options({  mode: "no-cors" })
        .post(JSON.stringify({message, signature}))
        .badRequest(async ({response}) => {
            console.log(await response.json());
        })
        .res();
}
export {sendSignature};