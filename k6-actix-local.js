import http from 'k6/http';
import {
  check
} from 'k6';
import crypto from 'k6/crypto';

export default function () {

  const randomContent = crypto.sha256(new Uint8Array(crypto.randomBytes(24)).buffer, 'hex')
  const url = 'http://localhost:8080/qrcode?content=' + randomContent;
  // console.log("calling url", url)
  let res = http.get(url)
  check(res, {
    'response is 200': (res) => res.status == 200
  });
}