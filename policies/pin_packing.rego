package xchemlab.pin_packing

import data.xchemlab
import future.keywords.if

default get_pin := {"allowed": false}

default create_pin := {"allow": false}

default get_puck := {"allowed": false}

default create_puck = {"allow": false}

default get_cane := {"allowed": false}

default create_cane := {"allowed": false}

get_pin= response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

create_pin= response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

get_puck = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

create_puck= response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

get_cane := response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

create_cane = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}
