#!/usr/bin/env nu

def assert [condition: bool, message: string] {
	if not $condition {
		error make { msg: $message }
	}
}

let rows = (net)
let columns = ($rows | columns)
let expected_columns = [name description if_index mac ips flags]
let missing_columns = ($expected_columns | where {|column| $column not-in $columns})
assert ($missing_columns | is-empty) $'missing expected columns: ($missing_columns | str join ", ")'

let projected = ($rows | where flags.is_up | select name if_index mac ips)
assert (($projected | columns) == [name if_index mac ips]) 'projection example did not return the expected columns'

let addresses = ($rows | each {|interface|
	{
		name: $interface.name,
		addresses: ($interface.ips | get addr),
	}
})
assert (($addresses | columns) == [name addresses]) 'nested address example did not return the expected columns'

let flattened = ($rows | flatten flags | select name if_index is_up is_loopback is_multicast)
assert (($flattened | columns) == [name if_index is_up is_loopback is_multicast]) 'flattened flags example did not return the expected columns'

print 'Validated nu_plugin_net examples with Nushell.'
