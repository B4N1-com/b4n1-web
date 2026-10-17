<?php

declare(strict_types=1);

namespace B4n1Web;

/**
 * Browser modes accepted by the b4n1web binary.
 */
enum BrowserMode: string
{
    case Light = 'light';
    case Js = 'js';
    case Render = 'render';
}
