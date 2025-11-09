Needed:

Creation of a "SAM Demo" user story

```sh
curl 'https://lateamrock.atlassian.net/rest/api/3/issue?updateHistory=true&applyDefaultValues=false' \
  -X POST \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0' \
  -H 'Accept: application/json,text/javascript,*/*' \
  -H 'Accept-Language: fr-FR,en-US;q=0.7,en;q=0.3' \
  -H 'Accept-Encoding: gzip, deflate, br, zstd' \
  -H 'Referer: https://lateamrock.atlassian.net/jira/software/c/projects/SAM/boards/1620' \
  -H 'Content-Type: application/json' \
  -H 'Origin: https://lateamrock.atlassian.net' \
  -H 'Sec-GPC: 1' \
  -H 'Connection: keep-alive' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'Priority: u=0' \
  -H 'TE: trailers' \
  --data-raw '{"fields":{"project":{"id":"10528"},"issuetype":{"id":"10001"},"summary":"Query sam from sam-demo using api-local","customfield_10020":3224,"versions":[{"id":"12594"}],"fixVersions":[{"id":"12594"}],"description":{"version":1,"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"Create an api-local endpoint in sam, and access it through a service in sam-demo."},{"type":"hardBreak"},{"type":"text","text":"Then create a api endpoint in sam-demo, and use it in the sam-demo frontend (dummy data is ok for now)"}]},{"type":"paragraph","content":[]},{"type":"paragraph","content":[{"type":"text","text":"Depends on:"}]},{"type":"bulletList","content":[{"type":"listItem","content":[{"type":"paragraph","content":[{"type":"inlineCard","attrs":{"url":"https://lateamrock.atlassian.net/browse/SAM-98"}}]}]},{"type":"listItem","content":[{"type":"paragraph","content":[{"type":"inlineCard","attrs":{"url":"https://lateamrock.atlassian.net/browse/SAM-100"}}]}]}]}]},"parent":{"id":"116727"},"components":[{"id":"10998","name":"SAM Demonstrator"}],"reporter":{"id":"712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d"},"priority":{"id":"10001","name":"Not prioritized","iconUrl":"https://lateamrock.atlassian.net/images/icons/priorities/trivial.svg"},"labels":[]},"update":{},"externalToken":"0.7494526191148907"}'
  ```

create sam demo bug

```sh
curl 'https://lateamrock.atlassian.net/rest/api/3/issue?updateHistory=true&applyDefaultValues=false' \
  -X POST \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0' \
  -H 'Accept: application/json,text/javascript,*/*' \
  -H 'Accept-Language: fr-FR,en-US;q=0.7,en;q=0.3' \
  -H 'Accept-Encoding: gzip, deflate, br, zstd' \
  -H 'Referer: https://lateamrock.atlassian.net/jira/software/c/projects/SAM/boards/1620' \
  -H 'Content-Type: application/json' \
  -H 'Origin: https://lateamrock.atlassian.net' \
  -H 'Sec-GPC: 1' \
  -H 'Connection: keep-alive' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'Priority: u=0' \
  -H 'TE: trailers' \
  --data-raw $'{"fields":{"project":{"id":"10528"},"issuetype":{"id":"10004"},"summary":"Missing redirection without trailing slash","customfield_10020":3224,"versions":[{"id":"12594"}],"fixVersions":[{"id":"12594"}],"description":{"version":1,"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"With or without authentication, accessing sam-demonstrator without trailing slash is not working."},{"type":"hardBreak"},{"type":"hardBreak"},{"type":"text","text":"- "},{"type":"text","text":"https://frl205143-vm/sam-demonstrator/","marks":[{"type":"code"}]},{"type":"text","text":" \u2192 OK"}]},{"type":"paragraph","content":[{"type":"text","text":"- "},{"type":"text","text":"https://frl205143-vm/sam-demonstrator","marks":[{"type":"code"}]},{"type":"text","text":" \u2192 KO, timeout and http:// redirection"}]}]},"parent":{"id":"116727"},"components":[{"id":"10998","name":"SAM Demonstrator"}],"reporter":{"id":"712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d"},"priority":{"id":"10001","name":"Not prioritized","iconUrl":"https://lateamrock.atlassian.net/images/icons/priorities/trivial.svg"},"labels":[]},"update":{},"externalToken":"0.621074196882546"}'
  ```


fetch board main info
```sh
curl -u "${env:JIRA_EMAIL}:${env:JIRA_API_TOKEN}" "https://requet.atlassian.net/rest/agile/1.0/board?projectKeyOrId=KAN" -H "Accept: application/json"
```

fetch epics
```sh
curl -u "${env:JIRA_EMAIL}:${env:JIRA_API_TOKEN}" "https://requet.atlassian.net/rest/agile/1.0/board/1/epic" -H "Accept: application/json"
```
fetch project meta data (issues types, bug etc id)
```sh
curl -u "${env:JIRA_EMAIL}:${env:JIRA_API_TOKEN}" "https://requet.atlassian.net/rest/api/3/project/KAN" -H "Accept: application/json"
```
