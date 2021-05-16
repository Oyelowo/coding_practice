import { useCallback, useEffect, useRef, useState } from 'react';
import { usePopper } from 'react-popper';


export interface ListNavigationValues<T> {
    selected: T | undefined;
    setSelected: React.Dispatch<React.SetStateAction<T | undefined>>;
    cursor: number;
    setCursor: React.Dispatch<React.SetStateAction<number>>;
    hovered: T | undefined;
    setHovered: React.Dispatch<React.SetStateAction<T | undefined>>;
    goUp: () => void;
    goDown: () => void;
    enter: () => void;
    getItemRef: (i: number) => (el: HTMLLIElement) => void;
    containerRef: React.MutableRefObject<HTMLUListElement | null>;
}

export function useListNavigation<T>(items: T[], onClose?: () => void) {
    const [selected, setSelected] = useState<T>();
    const [cursor, setCursor] = useState(0);
    const [hovered, setHovered] = useState<T>();

    const scroll = useScroll(cursor);

    useEffect(() => {
        setCursor(0);
    }, [items.length]);

    const goDown = useCallback(() => {
        setCursor((prevState) =>
            prevState < items.length - 1 ? prevState + 1 : 0,
        );
    }, [items.length]);

    const goUp = useCallback(() => {
        setCursor((prevState) =>
            prevState > 0 ? prevState - 1 : items.length - 1,
        );
    }, [items.length]);

    const enter = useCallback(() => {
        setSelected(items[cursor]);
        onClose?.();

        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [cursor, items.length, onClose]);

    useKeyboardShortcut('up', goUp);

    useKeyboardShortcut('down', goDown);

    useKeyboardShortcut('enter', enter);

    useKeyboardShortcut('esc', () => {
        onClose?.();
        setCursor(0);
    });


    useEffect(() => {
        const handleMouse = () => {
            setCursor(-1)
        }

        window.addEventListener('mousemove', handleMouse)
        return () => {
            window.removeEventListener('mousemove', handleMouse)
        }
    }, [])

    const props = {
        selected,
        setSelected,
        cursor,
        setCursor,
        hovered,
        setHovered,
        goUp,
        goDown,
        enter,
        ...scroll,
    };
    return props;
}

function useScroll(cursor: number) {
    const itemsRef = useRef<Array<Element | null>>([]);
    const containerRef = useRef<Element | null>(null);

    useEffect(() => {
        const activeItem = itemsRef.current[cursor];
        const elHeight = activeItem?.clientHeight ?? 0;
        const containerScrollTop = containerRef.current?.scrollTop ?? 0;
        const containerHeight = containerRef.current?.clientHeight ?? 0;

        const viewport = containerScrollTop + containerHeight;
        const elOffset = elHeight * cursor;

        const hasReachedBottomScroll = elOffset + elHeight > viewport;
        const hasReachedTopScroll = elOffset < containerScrollTop;

        if (
            (hasReachedBottomScroll || hasReachedTopScroll) &&
            containerRef.current
        ) {
            containerRef.current.scrollTop = elOffset;
        }
    }, [cursor, containerRef.current?.scrollTop]);

    const getItemRef = (itemIndex: number) => (el: HTMLElement) => {
        itemsRef.current[itemIndex] = el;
    };

    return {
        getItemRef,
        containerRef,
    };
}

export function usePopperValues() {
    const referenceRef = useRef<HTMLElement>(null);
    const popperRef = useRef<HTMLElement>(null);

    const { styles, attributes } = usePopper(
        referenceRef.current,
        popperRef.current,
        {
            placement: 'left-start',
            modifiers: [
                {
                    name: 'offset',
                    enabled: true,
                    options: {
                        offset: [5, 5],
                    },
                },
            ],
        },
    );

    return { referenceRef, popperRef, styles, attributes };
}
