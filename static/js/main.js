(function () {
  var xhr = new XMLHttpRequest();
    var request = document.getElementById("country").value;
     xhr.open("GET", "xml/" + request, true);
     xhr.onload = function (e) {
       var serverResponse = xhr.responseText;
       document.getElementById("prompt").innerHTML =
       "The time is currently " + serverResponse + ".";
     };
     xhr.onerror = function (e) {
       console.error(xhr.statusText);
     };
  xhr.send(null);

  setInterval(function(){
    xhr = new XMLHttpRequest();
    request = document.getElementById("country").value;
     xhr.open("GET", "xml/" + request, true);
     xhr.onload = function (e) {
       var serverResponse = xhr.responseText;
       document.getElementById("prompt").innerHTML =
       "The time is currently " + serverResponse + ".";
     };
     xhr.onerror = function (e) {
       console.error(xhr.statusText);
     };
     xhr.send(null);

  }, 1000);
})();
