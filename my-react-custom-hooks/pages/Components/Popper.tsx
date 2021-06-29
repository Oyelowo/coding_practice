const POPUP_ARROW_SIZE = 12;

const PopupArrow = styled.div`
  visibility: hidden;
  &,
  ::before {
    position: absolute;
    width: ${() => `${POPUP_ARROW_SIZE}`}px;
    height: ${() => `${POPUP_ARROW_SIZE}`}px;
    background: white;
  }
  &::before {
    visibility: visible;
    content: '';
    transform: rotate(45deg);
  }

  /*arrow positioning using popper.js data-popper-placement attribute*/
  &[data-popper-placement^='top'] {
    bottom: ${() => `-${POPUP_ARROW_SIZE / 2}`}px;

    ::before {
      box-shadow: 5px 5px 5px #eaeaea;
    }
  }

  &[data-popper-placement^='bottom'] {
    top: ${() => `-${POPUP_ARROW_SIZE / 2}`}px;
    ::before {
      box-shadow: -5px -5px 5px #eaeaea;
    }
  }

  &[data-popper-placement^='left'] {
    right: ${() => `-${POPUP_ARROW_SIZE / 2}`}px;
    ::before {
      box-shadow: 5px -5px 5px #eaeaea;
    }
  }

  &[data-popper-placement^='right'] {
    left: ${() => `-${POPUP_ARROW_SIZE / 2}`}px;
    ::before {
      box-shadow: -5px 5px 5px #eaeaea;
    }
  }
`;

/* 
  <div ref={popperRef} style={styles.popper} {...attributes.popper}>
        <PopupArrow ref={arrowRef} style={styles.arrow} {...attributes.popper} />
        <Popup />
 </div>
 */
