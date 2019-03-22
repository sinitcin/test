let dropArea = document.getElementById("drop-area")

;['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
  dropArea.addEventListener(eventName, preventDefaults, false)   
  document.body.addEventListener(eventName, preventDefaults, false)
})

;['dragenter', 'dragover'].forEach(eventName => {
  dropArea.addEventListener(eventName, highlight, false)
})

;['dragleave', 'drop'].forEach(eventName => {
  dropArea.addEventListener(eventName, unhighlight, false)
})

dropArea.addEventListener('drop', handleDrop, false)

function preventDefaults (e) {
  e.preventDefault()
  e.stopPropagation()
}

function highlight(e) {
  dropArea.classList.add('highlight')
}

function unhighlight(e) {
  dropArea.classList.remove('active')
}

function handleDrop(e) {
  var dt = e.dataTransfer
  var files = dt.files

  handleFiles(files)
}

let uploadProgress = []
let progressBar = document.getElementById('progress-bar')

function initializeProgress(numFiles) {
  progressBar.value = 0
  uploadProgress = []

  for(let i = numFiles; i > 0; i--) {
    uploadProgress.push(0)
  }
}

function updateProgress(fileNumber, percent) {
  uploadProgress[fileNumber] = percent
  let total = uploadProgress.reduce((tot, curr) => tot + curr, 0) / uploadProgress.length
  console.debug('update', fileNumber, percent, total)
  progressBar.value = total
}

function clearGalery() {
  for (var i = document.getElementById('gallery').childNodes.length - 1; i >= 0; i--) {
    document.getElementById('gallery').removeChild(document.getElementById('gallery').childNodes[i])
  }
}

function GetDataType(url) {
  if (url)
  {
     var base64 = url.toString().match(/data:image.([a-z]*);base64/);
     if (base64 && base64.length > 1)
     {
       return base64[1];
     }
  }
  return "";
}

function GetFileName(url)
{
   if (url)
   {
      var base64 = url.toString().match(/data:image.([a-z]*);base64/);
      if (base64 && base64.length > 1)
      {
        return base64[1];
      }
      var m = url.toString().match(/.*\/(.+?)\./);
      if (m && m.length > 1)
      {
        return m[1];
      }
   }
   return "";
}

function getFilesFromGallery() {
  let promises = [];
  const childNodes = document.querySelector('#gallery').childNodes
  let count = 0
  childNodes.forEach(function(node) {
      let imgUrl = node.src
      promises.push(fetch('https://cors-anywhere.herokuapp.com/' + imgUrl).then(function(response) {
        return response.blob();
      }).then(function(imgBlob){
          let filename = 'file_'+ count + '.' + GetFileName(imgUrl);
          count++;
          console.log(imgBlob)
          let file = new File([imgBlob], filename, {type: 'image/' + GetDataType(imgUrl)});
          return file
      }));
  });
  return Promise.all(promises);
}

function loadFromUrl() {  
  url = document.getElementById("urlEdit").value  
  let img = document.createElement('img')
  img.src = url
  document.getElementById('gallery').appendChild(img)
}

function handleFiles(files) {
  files = [...files]  
  initializeProgress(files.length)
  files.forEach(previewFile)
}

function upload() {
  let rMultipart = document.getElementById("RMultipart")
  if (rMultipart.checked == true) {
    uploadFiles();
  } else {
    uploadRest();
  }
  clearGalery();
}

function previewFile(file) {
  let reader = new FileReader()
  reader.readAsDataURL(file)
  reader.onloadend = function() {
    let img = document.createElement('img')
    img.src = reader.result
    document.getElementById('gallery').appendChild(img)
  }
}

function uploadFiles() { 

  getFilesFromGallery().then(
    function(files) {
      var formData = new FormData();
      files.forEach(
        function(file) {
          formData.append('images[]', file);
        }
      );
      var xhr = new XMLHttpRequest()
      xhr.open('POST', '/upload_multipart?img_count=' + files.length, true)
      xhr.setRequestHeader('X-Requested-With', 'XMLHttpRequest')
      /*
      xhr.upload.addEventListener("progress", function(e) {
        updateProgress(i, (e.loaded * 100.0 / e.total) || 100)
      });
      xhr.addEventListener('readystatechange', function(e) {
        if (xhr.readyState == 4 && xhr.status == 200) {
          updateProgress(i, 100)
        }
        else if (xhr.readyState == 4 && xhr.status != 200) {
          // Error
        }
      });
      */
      xhr.onreadystatechange = function() { 
        if (xhr.readyState != 4) return;
        alert(xhr.responseText);  
      };
      xhr.send(formData);
    }
  );
}

function uploadRest() {
  var xhr = new XMLHttpRequest()
  xhr.open('POST', '/upload_rest', true)
  xhr.setRequestHeader('Accept', 'application/json')
  xhr.setRequestHeader('Content-Type', 'application/json') 

  xhr.upload.addEventListener("progress", function(e) {
    updateProgress(i, (e.loaded * 100.0 / e.total) || 100)
  })
  xhr.addEventListener('readystatechange', function(e) {
    if (xhr.readyState == 4 && xhr.status == 200) {
      updateProgress(i, 100)
    }
    else if (xhr.readyState == 4 && xhr.status != 200) {
      // Error
    }
  })

  var filesData = '{"files": []}';
  var data = JSON.parse(filesData);
  for (var i = document.getElementById('gallery').childNodes.length - 1; i >= 0; i--) {
    data.files.push(document.getElementById('gallery').childNodes[i].src);
    //document.getElementById('gallery').removeChild(document.getElementById('gallery').childNodes[i])
  }
  xhr.send(JSON.stringify(data));
}