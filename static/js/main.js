(function () {
  setInterval(function(){
    var xhr = new XMLHttpRequest();
     xhr.open("GET", "xml/za", true);
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
