package envoy.authz

import future.keywords.if

default allow := false

graphql_sdl := http.send({
	"method": "get",
	"url": "http://backend/schema",
})

request_contents := json.unmarshal(replace(input.attributes.request.http.body, `\"`, `"`))
query_ast := graphql.parse(request_contents.query, graphql_sdl.raw_body)[0]

allow if input.attributes.request.http.path == "/"

allow if request_contents.operationName == "IntrospectionQuery"
