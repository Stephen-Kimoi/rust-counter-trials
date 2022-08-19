import { connect, Contract, keyStore, keyStores, WalletConnection } from 'near-api-js'; 
import getConfig from './config'; 

const nearConfig = getConfig(process.env.NODE_ENV || 'development'); 

// Initialize the contract and set global variables 
export async function initContract() {
    // Init connection to the near testnet 
    const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() }}, nearConfig))

    // Init a wallet based account 
    window.walletConnection = new WalletConnection(near) 

    // Get the account ID, returns an empty string if it's still unauthorized
    window.accountId = window.walletConnection.getAccountId()

    // Initializing our contract APIs by contract name and configuration 
    window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
        // View methods
        viewMethods: [ 'get_num'], 
        // Change methods, they modify the state 
        changeMethods: ['increament','decreament','reset']
    })
}

export function logOut() {
    window.walletConnection.signOut()
    // Reload the page 
    window.location.replace(window.location.origin + window.location.pathname)
}

export function logIn() {
    window.walletConnection.requestSignIn(nearConfig.contractName)
}

export async function getCounter() {
    let count = await window.contract.get_num({args:{}})
                                     .catch(error => errorHelper(error)) 
    return count; 
} 

export async function counterIncreament() {
    await window.contract.increament({args:{}}) 
}

export async function counterDecrement(){
    await window.contract.decrement({args:{}})
  }
  
export async function counterReset(){
await window.contract.reset({args:{}})
}

function errorHelper(err) {
    if (err.message.includes('Cannot deserialize the contract state')) {
      console.warn('NEAR Warning: the contract/account seems to have state that is not (or no longer) compatible.\n' +
          'This may require deleting and recreating the NEAR account as shown here:\n' +
          'https://stackoverflow.com/a/60767144/711863');
    }
    if (err.message.includes('Cannot deserialize the contract state')) {
      console.warn('NEAR Warning: the contract/account seems to have state that is not (or no longer) compatible.\n' +
          'This may require deleting and recreating the NEAR account as shown here:\n' +
          'https://stackoverflow.com/a/60767144/711863');
    }
    console.error(err)
}    