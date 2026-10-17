<?php

declare(strict_types=1);

namespace B4n1Web\Exception;

/**
 * Thrown when no b4n1web binary can be resolved (env/bundled/system/PATH).
 * Fails loudly - there are no silent fallbacks.
 */
class BinaryNotFoundException extends B4n1WebException
{
}
