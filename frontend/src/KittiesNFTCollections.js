import React, { useEffect, useState } from 'react'
import { Dropdown, Form, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittiesNFTCollectionCards from './KittiesNFTCollectionCards'

function toHexString(byteArray) {
  var s = '0x'
  byteArray.forEach(function (byte) {
    s += ('0' + (byte & 0xff).toString(16)).slice(-2)
  })
  return s
}

const parseKittyCollection = ({
  owner,
  ownerDeposit,
  items,
  itemMetadatas,
  itemConfigs,
  attributes,
}) => {
  return {
    owner: toHexString(owner),
    ownerDeposit: ownerDeposit.toHuman(),
    items: items.toHuman(),
    itemMetadatas: itemMetadatas.toHuman(),
    itemConfigs: itemConfigs.toHuman(),
    attributes: attributes.toHuman(),
  }
}

export default function KittiesNFTCollections(props) {
  const { api, keyring } = useSubstrateState()
  const [nextCollecitonId, setNextCollectionId] = useState(0)
  const [kittiesNftCollections, setKittiesNftCollections] = useState([])
  const [status, setStatus] = useState('')
  const [formState, setFormState] = useState({
    addressTo: '',
    maxSupply: 1000,
  })

  const accounts = keyring.getPairs()

  const onChange = (_, data) =>
    setFormState(prev => ({ ...prev, [data.state]: data.value }))

  const availableAccounts = []
  accounts.map(account => {
    return availableAccounts.push({
      key: account.meta.name,
      text: account.meta.name,
      value: account.address,
    })
  })

  const subscribeCount = () => {
    let unsub = null

    const asyncFetch = async () => {
      const fetchedKittiiesNFTCollections = []
      const nextCollectionIdRaw = await api.query.kittiesNFT.nextCollectionId()
      const parsedCollectionId = nextCollectionIdRaw.unwrap().toNumber()
      setNextCollectionId(parsedCollectionId)
      for (
        let collectionId = 0;
        collectionId < parsedCollectionId;
        collectionId++
      ) {
        const collectionDetail = await api.query.kittiesNFT.collection(
          collectionId
        )
        const dataEntries = collectionDetail.unwrap().entries()
        fetchedKittiiesNFTCollections.push(
          parseKittyCollection({
            owner: dataEntries.next().value[1],
            ownerDeposit: dataEntries.next().value[1],
            items: dataEntries.next().value[1],
            itemMetadatas: dataEntries.next().value[1],
            itemConfigs: dataEntries.next().value[1],
            attributes: dataEntries.next().value[1],
          })
        )
      }
      setKittiesNftCollections(fetchedKittiiesNFTCollections)
    }

    asyncFetch()

    return () => {
      unsub && unsub()
    }
  }

  useEffect(subscribeCount, [api, keyring])

  return (
    <Grid.Column width={16}>
      <h1>Kitties NFT Collection</h1>
      <h2>Number of collections: {nextCollecitonId}</h2>
      <KittiesNFTCollectionCards
        kittiesNFTCollections={kittiesNftCollections}
      />
      <Form style={{ margin: '1em 0' }}>
        <Form.Field
          style={{
            textAlign: 'center',
            display: 'flex',
            alignItems: 'center',
          }}
        >
          <label style={{ marginRight: 10 }}>Admin: </label>
          <Dropdown
            placeholder="Select from available addresses"
            fluid
            selection
            search
            options={availableAccounts}
            state="addressTo"
            onChange={onChange}
          />
        </Form.Field>
        <Form.Field
          style={{
            textAlign: 'center',
            display: 'flex',
            alignItems: 'center',
          }}
        >
          <label style={{ marginRight: 10 }}>MaxSupply: </label>
          <Form.Input
            placeholder="Enter max supply"
            fluid
            style={{ width: 150 }}
            value={formState.maxSupply}
            state="maxSupply"
            onChange={onChange}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            label="Create Kitty Collection"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'kittiesNFT',
              callable: 'create',
              inputParams: [formState.addressTo],
              paramFields: [true],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
    </Grid.Column>
  )
}
