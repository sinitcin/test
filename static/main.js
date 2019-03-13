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

function handleUrl(form) {  
  url = document.getElementById("urlEdit").value  
  let img = document.createElement('img')
  img.src = url
  document.getElementById('gallery').appendChild(img)
}

function handleFiles(files) {
  files = [...files]
  initializeProgress(files.length)
  files.forEach(uploadFile)
  files.forEach(previewFile)
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

function uploadFile(file, i) { 
  var xhr = new XMLHttpRequest()
  xhr.open('POST', '/upload_multipart', true)
  xhr.setRequestHeader('X-Requested-With', 'XMLHttpRequest')
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

  var formData = new FormData()
  formData.append('image', file)
  xhr.send(formData)
}

function uploadRest(files) {
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
    //alert(document.getElementById('gallery').childNodes[i].src);
    data.files.push(document.getElementById('gallery').childNodes[i].src);
    document.getElementById('gallery').removeChild(document.getElementById('gallery').childNodes[i])
  }
  xhr.send(JSON.stringify(data));
}