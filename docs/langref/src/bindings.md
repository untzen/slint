## Bindings

The binding expression is automatically re-evaluated when properties accessed in the expression change.

In the following example, the text of the button automatically changes when
the user presses the button. Incrementing the `counter` property automatically
invalidates the expression bound to `text` and triggers a re-evaluation.

```slint
import { Button } from "std-widgets.slint";
export component Example inherits Window {
    preferred-width: 50px;
    preferred-height: 50px;
    Button {
        property <int> counter: 3;
        clicked => { self.counter += 3 }
        text: self.counter * 2;
    }
}
```

The re-evaluation happens lazily when the property is queried.

Internally, a dependency is registered for any property accessed while evaluating a binding.
When a property changes, the dependencies are notified and all dependent bindings
are marked as dirty.

Callbacks in native code by default don't depend on any properties unless they query a property in the native code.