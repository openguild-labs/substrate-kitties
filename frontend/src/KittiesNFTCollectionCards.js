import React from 'react'
import { Card, Grid, Message, Label } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'

// --- About Kitty Card ---

const KittiesNFTCollectionCard = props => {
  const { kittyNFTCollection } = props
  const {
    owner = null,
    ownerDeposit = null,
    items = null,
    itemMetadatas = null,
    itemConfigs = null,
    attributes = null,
  } = kittyNFTCollection
  const { currentAccount } = useSubstrateState()
  const isSelf = currentAccount.address === kittyNFTCollection.owner

  return (
    <Card>
      {isSelf && (
        <Label as="a" floating color="teal">
          Mine
        </Label>
      )}
      {/* <KittyAvatar dna={dna.toU8a()} /> */}
      <Card.Content>
        <Card.Meta style={{ fontSize: '.9em', overflowWrap: 'break-word' }}>
          Collection Admin: {owner}
        </Card.Meta>
        <Card.Description>
          <p style={{ overflowWrap: 'break-word' }}>
            Owner Deposit: {ownerDeposit}
          </p>
          <p style={{ overflowWrap: 'break-word' }}>Item Count: {items}</p>
          <p style={{ overflowWrap: 'break-word' }}>
            Item Config Count: {itemConfigs}
          </p>
          <p style={{ overflowWrap: 'break-word' }}>
            Item Metadata Count: {itemMetadatas}
          </p>
          <p style={{ overflowWrap: 'break-word' }}>
            Attribute Count : {attributes}
          </p>
        </Card.Description>
      </Card.Content>
      <Card.Content extra style={{ textAlign: 'center' }}></Card.Content>
    </Card>
  )
}

const KittiesNFTCollectionCards = props => {
  const { kittiesNFTCollections, setStatus } = props

  if (kittiesNFTCollections.length === 0) {
    return (
      <Message info>
        <Message.Header>
          No Kitty MFT Collections found here... Create one now!&nbsp;
          <span role="img" aria-label="point-down">
            👇
          </span>
        </Message.Header>
      </Message>
    )
  }

  return (
    <Grid columns={3}>
      {kittiesNFTCollections.map((kittyNFTCollection, i) => (
        <Grid.Column key={`kitty-nft-collection-${i}`}>
          <KittiesNFTCollectionCard
            kittyNFTCollection={kittyNFTCollection}
            setStatus={setStatus}
          />
        </Grid.Column>
      ))}
    </Grid>
  )
}

export default KittiesNFTCollectionCards
