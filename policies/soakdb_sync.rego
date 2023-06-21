package xchemlab.soakdb_sync

import data.xchemlab
import future.keywords.if

default read := {"allowed": false}

default update_metadata := {"allowed": false}

default insert_wells := {"allowed": false}

read = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

update_metadata := response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

insert_wells = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}
