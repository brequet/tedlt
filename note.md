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

get a single ticket data

```sh
curl -u "${env:JIRA_EMAIL}:${env:JIRA_API_TOKEN}" "https://lateamrock.atlassian.net/rest/api/3/issue/SAM-102" -H "Accept: application/json"
```

```json
{
  "expand": "renderedFields,names,schema,operations,editmeta,changelog,versionedRepresentations,customfield_10010.requestTypePractice,customfield_10089.properties",
  "id": "118307",
  "self": "https://lateamrock.atlassian.net/rest/api/3/issue/118307",
  "key": "SAM-102",
  "fields": {
    "statuscategorychangedate": "2025-11-10T15:47:42.072+0100",
    "parent": {
      "id": "116727",
      "key": "SAM-9",
      "self": "https://lateamrock.atlassian.net/rest/api/3/issue/116727",
      "fields": {
        "summary": "Tools and Libraries",
        "status": {
          "self": "https://lateamrock.atlassian.net/rest/api/3/status/10000",
          "description": "",
          "iconUrl": "https://lateamrock.atlassian.net/",
          "name": "Backlog",
          "id": "10000",
          "statusCategory": {
            "self": "https://lateamrock.atlassian.net/rest/api/3/statuscategory/2",
            "id": 2,
            "key": "new",
            "colorName": "blue-gray",
            "name": "A faire"
          }
        },
        "priority": {
          "self": "https://lateamrock.atlassian.net/rest/api/3/priority/10001",
          "iconUrl": "https://lateamrock.atlassian.net/images/icons/priorities/trivial.svg",
          "name": "Not prioritized",
          "id": "10001"
        },
        "issuetype": {
          "self": "https://lateamrock.atlassian.net/rest/api/3/issuetype/10000",
          "id": "10000",
          "description": "Une collection de bugs, stories et tâches connexes.",
          "iconUrl": "https://lateamrock.atlassian.net/images/icons/issuetypes/epic.svg",
          "name": "Epic",
          "subtask": false,
          "hierarchyLevel": 1
        }
      }
    },
    "customfield_10074": null,
    "customfield_10197": null,
    "customfield_10198": null,
    "fixVersions": [
      {
        "self": "https://lateamrock.atlassian.net/rest/api/3/version/12594",
        "id": "12594",
        "description": "",
        "name": "Phase 1",
        "archived": false,
        "released": false,
        "releaseDate": "2026-05-31"
      }
    ],
    "statusCategory": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/statuscategory/4",
      "id": 4,
      "key": "indeterminate",
      "colorName": "yellow",
      "name": "En cours"
    },
    "customfield_10199": null,
    "resolution": null,
    "customfield_10113": null,
    "customfield_10105": null,
    "customfield_10106": null,
    "lastViewed": "2025-11-10T16:17:42.181+0100",
    "customfield_10060": null,
    "customfield_10061": null,
    "customfield_11271": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/customFieldOption/11175",
      "value": "VERIF",
      "id": "11175"
    },
    "customfield_10064": null,
    "customfield_10065": null,
    "customfield_10066": null,
    "customfield_10100": null,
    "customfield_10980": null,
    "priority": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/priority/10001",
      "iconUrl": "https://lateamrock.atlassian.net/images/icons/priorities/trivial.svg",
      "name": "Not prioritized",
      "id": "10001"
    },
    "customfield_10067": null,
    "customfield_10981": null,
    "customfield_10982": null,
    "customfield_10069": null,
    "customfield_10102": null,
    "labels": [],
    "customfield_10983": null,
    "customfield_10974": null,
    "customfield_10975": null,
    "customfield_10976": null,
    "customfield_10977": null,
    "timeestimate": null,
    "aggregatetimeoriginalestimate": null,
    "versions": [],
    "customfield_10978": null,
    "customfield_10979": null,
    "issuelinks": [
      {
        "id": "62609",
        "self": "https://lateamrock.atlassian.net/rest/api/3/issueLink/62609",
        "type": {
          "id": "10006",
          "name": "Work item split",
          "inward": "split from",
          "outward": "split to",
          "self": "https://lateamrock.atlassian.net/rest/api/3/issueLinkType/10006"
        },
        "inwardIssue": {
          "id": "116849",
          "key": "SAM-45",
          "self": "https://lateamrock.atlassian.net/rest/api/3/issue/116849",
          "fields": {
            "summary": "Setup Angular",
            "status": {
              "self": "https://lateamrock.atlassian.net/rest/api/3/status/10006",
              "description": "",
              "iconUrl": "https://lateamrock.atlassian.net/images/icons/statuses/generic.png",
              "name": "Revue en cours",
              "id": "10006",
              "statusCategory": {
                "self": "https://lateamrock.atlassian.net/rest/api/3/statuscategory/4",
                "id": 4,
                "key": "indeterminate",
                "colorName": "yellow",
                "name": "En cours"
              }
            },
            "priority": {
              "self": "https://lateamrock.atlassian.net/rest/api/3/priority/10001",
              "iconUrl": "https://lateamrock.atlassian.net/images/icons/priorities/trivial.svg",
              "name": "Not prioritized",
              "id": "10001"
            },
            "issuetype": {
              "self": "https://lateamrock.atlassian.net/rest/api/3/issuetype/10001",
              "id": "10001",
              "description": "Une fonctionnalité exprimée sous la forme d'un objectif utilisateur.",
              "iconUrl": "https://lateamrock.atlassian.net/rest/api/2/universal_avatar/view/type/issuetype/avatar/10315?size=medium",
              "name": "Story",
              "subtask": false,
              "avatarId": 10315,
              "hierarchyLevel": 0
            }
          }
        }
      }
    ],
    "assignee": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A150e46dc-b6bc-4b66-bc21-f05714f93e3d",
      "accountId": "712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d",
      "emailAddress": "baptiste.requet@ext.biomerieux.com",
      "avatarUrls": {
        "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d/bfb54b77-7e29-49f0-804c-1daa44dd1dea/48",
        "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d/bfb54b77-7e29-49f0-804c-1daa44dd1dea/24",
        "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d/bfb54b77-7e29-49f0-804c-1daa44dd1dea/16",
        "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:150e46dc-b6bc-4b66-bc21-f05714f93e3d/bfb54b77-7e29-49f0-804c-1daa44dd1dea/32"
      },
      "displayName": "Baptiste REQUET",
      "active": true,
      "timeZone": "Europe/Paris",
      "accountType": "atlassian"
    },
    "status": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/status/3",
      "description": "Ce ticket est en cours de traitement par la personne assignée.",
      "iconUrl": "https://lateamrock.atlassian.net/images/icons/statuses/inprogress.png",
      "name": "En cours",
      "id": "3",
      "statusCategory": {
        "self": "https://lateamrock.atlassian.net/rest/api/3/statuscategory/4",
        "id": 4,
        "key": "indeterminate",
        "colorName": "yellow",
        "name": "En cours"
      }
    },
    "components": [
      {
        "self": "https://lateamrock.atlassian.net/rest/api/3/component/10998",
        "id": "10998",
        "name": "SAM Demonstrator"
      }
    ],
    "customfield_10052": null,
    "customfield_10058": null,
    "customfield_10059": null,
    "customfield_11535": null,
    "customfield_10203": [],
    "customfield_10204": null,
    "customfield_10205": null,
    "customfield_10206": null,
    "customfield_10207": null,
    "aggregatetimeestimate": null,
    "customfield_10208": null,
    "creator": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
      "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
      "emailAddress": "julien.biaggi@biomerieux.com",
      "avatarUrls": {
        "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
        "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
        "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
        "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
      },
      "displayName": "Julien BIAGGI",
      "active": true,
      "timeZone": "Europe/Paris",
      "accountType": "atlassian"
    },
    "subtasks": [],
    "customfield_11370": null,
    "reporter": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
      "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
      "emailAddress": "julien.biaggi@biomerieux.com",
      "avatarUrls": {
        "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
        "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
        "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
        "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
      },
      "displayName": "Julien BIAGGI",
      "active": true,
      "timeZone": "Europe/Paris",
      "accountType": "atlassian"
    },
    "aggregateprogress": { "progress": 0, "total": 0 },
    "customfield_10200": null,
    "customfield_10201": null,
    "customfield_10202": null,
    "customfield_10038": [],
    "customfield_11404": null,
    "progress": { "progress": 0, "total": 0 },
    "votes": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/issue/SAM-102/votes",
      "votes": 0,
      "hasVoted": false
    },
    "worklog": { "startAt": 0, "maxResults": 20, "total": 0, "worklogs": [] },
    "issuetype": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/issuetype/10001",
      "id": "10001",
      "description": "Une fonctionnalité exprimée sous la forme d'un objectif utilisateur.",
      "iconUrl": "https://lateamrock.atlassian.net/rest/api/2/universal_avatar/view/type/issuetype/avatar/10315?size=medium",
      "name": "Story",
      "subtask": false,
      "avatarId": 10315,
      "hierarchyLevel": 0
    },
    "timespent": null,
    "project": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/project/10528",
      "id": "10528",
      "key": "SAM",
      "name": "Smart AST Module",
      "projectTypeKey": "software",
      "simplified": false,
      "avatarUrls": {
        "48x48": "https://lateamrock.atlassian.net/rest/api/3/universal_avatar/view/type/project/avatar/10917",
        "24x24": "https://lateamrock.atlassian.net/rest/api/3/universal_avatar/view/type/project/avatar/10917?size=small",
        "16x16": "https://lateamrock.atlassian.net/rest/api/3/universal_avatar/view/type/project/avatar/10917?size=xsmall",
        "32x32": "https://lateamrock.atlassian.net/rest/api/3/universal_avatar/view/type/project/avatar/10917?size=medium"
      }
    },
    "aggregatetimespent": null,
    "customfield_10029": null,
    "resolutiondate": null,
    "workratio": -1,
    "watches": {
      "self": "https://lateamrock.atlassian.net/rest/api/3/issue/SAM-102/watchers",
      "watchCount": 1,
      "isWatching": false
    },
    "issuerestriction": { "issuerestrictions": {}, "shouldDisplay": false },
    "created": "2025-11-03T15:07:44.344+0100",
    "customfield_10020": [
      {
        "id": 3224,
        "name": "Phase 1 - Sprint 2",
        "state": "active",
        "boardId": 1620,
        "goal": "Display information on SAM Demo from SAM",
        "startDate": "2025-11-04T15:19:53.185Z",
        "endDate": "2025-11-12T01:30:00.000Z"
      }
    ],
    "customfield_10021": null,
    "customfield_10022": null,
    "customfield_10023": null,
    "customfield_10016": null,
    "customfield_10017": null,
    "customfield_10019": "0|i0m0vm:",
    "updated": "2025-11-10T16:17:25.519+0100",
    "customfield_10090": null,
    "timeoriginalestimate": null,
    "description": {
      "type": "doc",
      "version": 1,
      "content": [
        {
          "type": "paragraph",
          "content": [
            { "type": "text", "text": "Dépend de " },
            {
              "type": "inlineCard",
              "attrs": {
                "url": "https://lateamrock.atlassian.net/browse/SAM-45"
              }
            },
            { "type": "text", "text": " " }
          ]
        },
        {
          "type": "paragraph",
          "content": [{ "type": "text", "text": "Basic angular module" }]
        },
        {
          "type": "paragraph",
          "content": [{ "type": "text", "text": "Use index.html by default" }]
        },
        {
          "type": "paragraph",
          "content": [
            {
              "type": "text",
              "text": "deploy angular code into jar and installer"
            }
          ]
        }
      ]
    },
    "customfield_10010": null,
    "customfield_10014": "SAM-9",
    "timetracking": {},
    "customfield_10015": null,
    "customfield_10005": null,
    "customfield_10007": null,
    "security": null,
    "customfield_10008": null,
    "customfield_11338": null,
    "attachment": [],
    "customfield_10009": null,
    "summary": "Integration of bmx-lib",
    "customfield_10087": "",
    "customfield_10000": "{}",
    "customfield_10088": 0.0,
    "customfield_10089": null,
    "customfield_10001": null,
    "customfield_10002": [],
    "customfield_10125": "\r\n⚠ Before moving to 'In review', make sure ALL expected activities have been completed, and ready to be reviewed. In case of review needed during development, leave it 'In progress'.",
    "environment": null,
    "customfield_10118": null,
    "customfield_10119": null,
    "duedate": null,
    "comment": {
      "comments": [
        {
          "self": "https://lateamrock.atlassian.net/rest/api/3/issue/118307/comment/373013",
          "id": "373013",
          "author": {
            "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "emailAddress": "julien.biaggi@biomerieux.com",
            "avatarUrls": {
              "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
              "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
              "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
              "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
            },
            "displayName": "Julien BIAGGI",
            "active": true,
            "timeZone": "Europe/Paris",
            "accountType": "atlassian"
          },
          "body": {
            "type": "doc",
            "version": 1,
            "content": [
              {
                "type": "paragraph",
                "content": [
                  {
                    "type": "text",
                    "text": "Baptiste REQUET",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/baptiste.requet"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " mentioned this issue in " },
                  {
                    "type": "text",
                    "text": "a commit",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo/-/commit/cb5337f3b6113bf0914e1d2dcb34932dc389d60e"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " of " },
                  {
                    "type": "text",
                    "text": "bioMérieux / LABINFO / SAM / SAM Demonstrator",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " on branch " },
                  {
                    "type": "text",
                    "text": "sam-102-bmx-lib",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo/-/tree/sam-102-bmx-lib"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": ":" }
                ]
              },
              {
                "type": "blockquote",
                "content": [
                  {
                    "type": "paragraph",
                    "content": [
                      {
                        "type": "inlineCard",
                        "attrs": {
                          "url": "https://lateamrock.atlassian.net/browse/SAM-102#icft=SAM-102"
                        }
                      },
                      {
                        "type": "text",
                        "text": " build(frontend): add angular bmx-library"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          "updateAuthor": {
            "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "emailAddress": "julien.biaggi@biomerieux.com",
            "avatarUrls": {
              "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
              "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
              "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
              "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
            },
            "displayName": "Julien BIAGGI",
            "active": true,
            "timeZone": "Europe/Paris",
            "accountType": "atlassian"
          },
          "created": "2025-11-10T16:15:45.983+0100",
          "updated": "2025-11-10T16:15:45.983+0100",
          "jsdPublic": true
        },
        {
          "self": "https://lateamrock.atlassian.net/rest/api/3/issue/118307/comment/373014",
          "id": "373014",
          "author": {
            "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "emailAddress": "julien.biaggi@biomerieux.com",
            "avatarUrls": {
              "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
              "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
              "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
              "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
            },
            "displayName": "Julien BIAGGI",
            "active": true,
            "timeZone": "Europe/Paris",
            "accountType": "atlassian"
          },
          "body": {
            "type": "doc",
            "version": 1,
            "content": [
              {
                "type": "paragraph",
                "content": [
                  {
                    "type": "text",
                    "text": "Baptiste REQUET",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/baptiste.requet"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " mentioned this issue in " },
                  {
                    "type": "text",
                    "text": "a merge request",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo/-/merge_requests/39"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " of " },
                  {
                    "type": "text",
                    "text": "bioMérieux / LABINFO / SAM / SAM Demonstrator",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": " on branch " },
                  {
                    "type": "text",
                    "text": "sam-102-bmx-lib",
                    "marks": [
                      {
                        "type": "link",
                        "attrs": {
                          "href": "https://gitlab.com/biomerieux-sa/labinfo/sam/sam-demo/-/tree/sam-102-bmx-lib"
                        }
                      }
                    ]
                  },
                  { "type": "text", "text": ":" }
                ]
              },
              {
                "type": "blockquote",
                "content": [
                  {
                    "type": "paragraph",
                    "content": [
                      {
                        "type": "inlineCard",
                        "attrs": {
                          "url": "https://lateamrock.atlassian.net/browse/SAM-102#icft=SAM-102"
                        }
                      },
                      {
                        "type": "text",
                        "text": " build(frontend): add angular bmx-library"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          "updateAuthor": {
            "self": "https://lateamrock.atlassian.net/rest/api/3/user?accountId=712020%3A86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "accountId": "712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c",
            "emailAddress": "julien.biaggi@biomerieux.com",
            "avatarUrls": {
              "48x48": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/48",
              "24x24": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/24",
              "16x16": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/16",
              "32x32": "https://avatar-management--avatars.us-west-2.prod.public.atl-paas.net/712020:86e88fa3-041e-4404-aed2-1abf83bdfb3c/a2fa8efc-9183-4408-bc22-dd43cff6b2da/32"
            },
            "displayName": "Julien BIAGGI",
            "active": true,
            "timeZone": "Europe/Paris",
            "accountType": "atlassian"
          },
          "created": "2025-11-10T16:17:24.999+0100",
          "updated": "2025-11-10T16:17:24.999+0100",
          "jsdPublic": true
        }
      ],
      "self": "https://lateamrock.atlassian.net/rest/api/3/issue/118307/comment",
      "maxResults": 2,
      "total": 2,
      "startAt": 0
    }
  }
}
```
