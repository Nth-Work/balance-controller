# Balances Controller

## Description

This is a micro-service developed by [Nth.work](nth.work) to manager user balances in a secure way

## Available endpoints
<pre>GET "/{user_id}"
Get balance of user</pre>

<pre> POST "/add_user"
Create a new user

BODY {
    user_id: String,
    value: usize
} 
</pre>


<pre> POST "/add"
Add LOCKED balance to user account
BODY {
    user_id: String,
    value: usize
}
</pre>
<pre> POST "/unlock"
Unlock balance of user account
BODY {
    user_id: String,
    value: usize
}
</pre>
<pre> POST "/lock"
Lock balance of user account
BODY {
    user_id: String,
    value: usize
}
</pre>
<pre> POST "/remove"
Remove LOCKED balance of user account
BODY {
    user_id: String,
    value: usize
}
</pre>
<pre> POST "/force_add"
Add FREE balance DRECTLY to user account
BODY {
    user_id: String,
    value: usize
}
</pre>
<pre> POST "/force_remove"
Remove FREE balance DRECTLY to user account
BODY {
    user_id: String,
    value: usize
}
</pre>