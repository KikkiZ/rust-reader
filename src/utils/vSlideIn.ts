import { Directive } from "vue";

const DISTANCE = 150;
const DURATION = 500;

const slideDownMap = new WeakMap();
const slideUpMap = new WeakMap();

const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
        if (!entry.isIntersecting) return;

        if (fromAboveViewport(entry)) {
            const animation = slideDownMap.get(entry.target);
            animation.play();
        }

        if (fromBelowViewport(entry)) {
            const animation = slideUpMap.get(entry.target);
            animation.play();
        }
    });
});

function fromAboveViewport(entry: IntersectionObserverEntry) {
    const rect = entry.boundingClientRect;
    return rect.top < 0;
}

function fromBelowViewport(entry: IntersectionObserverEntry) {
    const rect = entry.boundingClientRect;
    return rect.bottom > window.innerHeight;
}

const slideIn: Directive = {
    mounted(element: HTMLElement) {
        // 添加自定义动画
        const slideDown = element.animate(
            [
                {
                    transform: `translateY(-${DISTANCE}px)`,
                    opacity: 0.5,
                },
                {
                    transform: "translateY(0)",
                    opacity: 1,
                },
            ],
            {
                duration: DURATION,
                easing: "ease-in-out",
            },
        );
        slideDownMap.set(element, slideDown);

        const slideUp = element.animate(
            [
                {
                    transform: `translateY(${DISTANCE}px)`,
                    opacity: 0.5,
                },
                {
                    transform: "translateY(0)",
                    opacity: 1,
                },
            ],
            {
                duration: DURATION,
                easing: "ease-in-out",
            },
        );
        slideUpMap.set(element, slideUp);

        observer.observe(element);
    },

    unmounted(element: HTMLElement) {
        observer.unobserve(element);
    },
};

export default slideIn;
