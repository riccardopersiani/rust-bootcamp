## Error Handling

Things are really complicated, there is a dedicated error handling group working in Rust error handling. 
Despite these crates the complicacy is high. 

There is also ErrorStack, check to see the custom errors. 

There are example with great context for errors comparing to the standard. 

How is different from anyhow nad eyre. They want to encourage new errors when cross module.

Also ability to attach data to an error without a lot of configuration. 

Report = stack of frames, Context or Attachment.
Context 
Attachment: extra. 

Report iterate the Frame stack producing a good error message. 