import { Modal, ModalBody, ModalContent } from '@nextui-org/react'
import * as React from 'react'
import { createRoot } from 'react-dom/client'


const open = (props: {
    title: string,
    content: string,
    type: 'success' | 'error' | 'info' | 'warning',
}) => {
    const root = createRoot(document.getElementById('base-modal') as HTMLElement);
    const Totast = (props: {
        title: string,
        content: string,
        type: 'success' | 'error' | 'info' | 'warning',
    }) => {
        const [visible, setVisible] = React.useState(false)
    
        React.useEffect(() => {
            setVisible(true)
            setTimeout(() => {
                setVisible(false)
                root.unmount()
            }, 3000)
        }, [])
        return (
            <Modal isOpen={visible}
                placement='top'
                size='sm'
            >
                <ModalContent>
                    <ModalBody>
                        <h3 className={'text-xl font-bold'}>{props.title}</h3>
                        <p className={'text-sm'}>{props.content}</p>
                    </ModalBody>
                </ModalContent>
            </Modal>
        )
    }
    root.render(<Totast {...props} />)
}

export default {
    open   
}