import wretch from 'wretch';
import {auth} from './auth';
async function sendSignature(message, signature) {
    let res = await wretch()
        .url(
            'http://localhost:8000/auth'
        )
        .content('application/json')
        // .options({  mode: "no-cors" })
        .post(JSON.stringify({message, signature}))
        .badRequest(async ({response}) => {
            console.log(await response.json());
        })
        .res(async (response) => {
            const data = await response.json()
            console.log(data)
            auth.set(data.token)
        });

}
export {sendSignature};