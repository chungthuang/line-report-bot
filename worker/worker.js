addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { collect_report } = wasm_bindgen;
  await wasm_bindgen(wasm)

  const botConfig = {
    channel_secret: CHANNEL_SECRET,
    channel_access_token: CHANNEL_ACCESS_TOKEN,
  }
  result = await collect_report(request, botConfig)
    .then(result => {
      return result
    })
    .catch(err => {
      console.log("greet failed because ", err)
      return new Response("collect_report failed", { status: 200 })
    })
  return new Response("collect_report succeed", { status: 200 })
}
