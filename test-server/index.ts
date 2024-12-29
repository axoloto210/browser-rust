import http from "http";
import fs from "fs";

import path from "path";

const port = 8000;

const HTML_FILE = "test.html";

const server = http.createServer((req, res) => {
  //  // リクエストの詳細をログ出力
  //  console.log('\n=== リクエスト詳細 ===');
  //  console.log(`Method: ${req.method}`);
  //  console.log(`URL: ${req.url}`);
  //  console.log(`HTTP Version: ${req.httpVersion}`);
  //  console.log('\n=== リクエストヘッダー ===');
  //  console.log(req.headers);

  if (req.url === `/${HTML_FILE}`) {
    const filePath = path.join(__dirname, HTML_FILE);

    fs.readFile(filePath, (err, data) => {
      if (err) {
        res.writeHead(500);
        res.end(`Error loading ${HTML_FILE}: ${err}`);
        return;
      }

      res.writeHead(200, {
        "content-type": "text/html",
      });
      res.end(data);
    });
  } else {
    res.writeHead(404);
    res.end("Not Found");
  }
});

server.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}/`);
});
