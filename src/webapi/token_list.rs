use webcore::value::Reference;
use webcore::try_from::TryInto;
use private::TODO;

/// The `TokenList` represents a set of space-separated tokens.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)
// https://dom.spec.whatwg.org/#domtokenlist
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DOMTokenList")]
pub struct TokenList( Reference );

impl TokenList {
    /// Gets the number of tokens in the list.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-length
    pub fn len( &self ) -> u32 {
        js!( return @{self}.length; ).try_into().unwrap()
    }

    /// Adds token to the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-add
    pub fn add( &self, token: &str ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.add( @{token} );
        }

        Ok(())
    }

    /// Removes token from the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-remove
    pub fn remove( &self, token: &str ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.remove( @{token} );
        }

        Ok(())
    }

    /// The toggle() method of the DOMTokenList interface removes a given token from the list and returns false. If token doesn't exist it's added and the function returns true.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)
    // https://dom.spec.whatwg.org/#dom-domtokenlist-toggle
    pub fn toggle( &self, token: &str ) -> Result< bool, TODO > {
        Ok( js!( return @{self}.toggle( @{token} ); ).try_into().unwrap() )
    }

    /// This method is like toggle but a one-way operation. If force is set to `false`,
    /// the token will only be removed but not added again. If set to `true`, the token
    /// will only be added but not removed again.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)
    // https://dom.spec.whatwg.org/#dom-domtokenlist-toggle
    pub fn toggle_force( &self, token: &str, force: bool ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.toggle( @{token}, @{force} );
        }

        Ok(())
    }

    /// Returns `true` if the underlying string contains token, otherwise `false`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-contains
    pub fn contains( &self, token: &str ) -> bool {
        js!( return @{self}.contains( @{token} ); ).try_into().unwrap()
    }
}
