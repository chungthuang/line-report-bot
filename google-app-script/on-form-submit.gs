// e: Form submit event https://developers.google.com/apps-script/guides/triggers/events#google_forms_events
function onFormSubmit(e) {
  Logger.log("[METHOD] onFormSubmit, event: ", e);
  // https://developers.google.com/apps-script/guides/triggers/events#google_forms_events
  callWorker(e.response);
}

// Notifies worker of a new submission
function callWorker(formResp) {
  var scriptProperties = PropertiesService.getScriptProperties();
  const workerUrl = scriptProperties.getProperty("workerUrl"); 
  Logger.log("Worker url %s", workerUrl);
  var options = {
    'method' : 'post',
    'contentType': 'text/plain',
    'payload' : payload(formResp.getItemResponses()),
  };
  var resp = UrlFetchApp.fetch(workerUrl, options);
  Logger.log("Submission result %s", resp);
}

function payload(itemResps) {
  var data = '';
  for (var i = 0; i < itemResps.length; i++) {
    var itemResp = itemResps[i];
    data += itemResp.getItem().getTitle() + ': ' + itemResp.getResponse() + '\n';
  }
  return data
}
