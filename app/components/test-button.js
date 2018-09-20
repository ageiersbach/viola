import Component from '@ember/component';
import { inject } from '@ember/service';

export default Component.extend({
  violaConnect: inject(),

  actions: {
    doClick: function() {
      this.get('violaConnect').call([], function(err, data) {
        console.log('data?', data);
        console.log('err?', err);
      });
    }
  }
});
