/*
Script to perform an e2e test of the service with k6 (https://k6.io)
*/
import http from 'k6/http';
import {
  check
} from 'k6';
import crypto from 'k6/crypto';

//TODO use Options for the url, the length of the randomContent, etc

export default function () {

  const randomContent = crypto.sha256(new Uint8Array(crypto.randomBytes(24)).buffer, 'hex')
  const url = 'http://localhost:8080/?content=' + randomContent;
  // console.log("calling url", url)
  let res = http.get(url)
  check(res, {
    'response is 200': (res) => res.status == 200
  });
}