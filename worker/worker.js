addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { greet } = wasm_bindgen;
  await wasm_bindgen(wasm)

  const body = await request.json()
    .then(data => {
      return data;
    });

  botConfig = {
    channel_secret: CHANNEL_SECRET,
  }
  const greeting = greet(body, request.headers, botConfig)
    .then(result => {
      return result
    })
    .catch(err => {
      console.log("greet failed because ", err)
      return new Response("your signature is invalid", { status: 200 })
    })
  return new Response("recognize signature", { status: 200 })
}
