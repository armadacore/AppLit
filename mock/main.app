import route:{default, routeA, routeB} from '/route';

@id('devId', 'appId');
@icon('/assets/icon.png');
@name('myApp');
@version('0.1.0');
@description('demo app to prove compiler');
@link('https://www.armadacore.com');
@domain(route.default, {
    routeA,
    routeB,
});