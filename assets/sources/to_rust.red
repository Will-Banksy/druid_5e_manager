Red [
	Title: "D&D JSON to Rust struct deriving serde"
]

; TODO: Automatically populate the arrays
sources: [
	[ ; WOTC SRD
		"wotc_srd/armor.json"
		"wotc_srd/feats.json"
	]
]

foreach source sources [
	foreach file source [
		contents: load/as read to-file file 'json
		foreach item contents [
			; print type? item ; Okay each json item is a map... how do I iterate over this now. foreach doesn't seem to work...
			print item
		]
	]
]
