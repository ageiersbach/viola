import Service from '@ember/service';
const child = requireNode('child_process').execFile;
const execPath = '../../target/debug/viola.exe';

export default Service.extend({

  call: function(args, callback) {
    return child(execPath, args, callback);
  }
});
